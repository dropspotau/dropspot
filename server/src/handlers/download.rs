use axum::{
    body::Body,
    extract::{Json, Path, State},
    response::{IntoResponse, Response},
};
use dropspot_core::download::Download as ApiDownload;
use reqwest::StatusCode;
use thiserror::Error;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::db::{Download, User, create_download, get_download_by_id, get_file_by_id};
use crate::state::AppState;
use crate::types::ApiError;

#[derive(Error, Debug)]
pub enum FileDownloadError {
    #[error("File not found")]
    FileNotFound,

    #[error("File has expired")]
    FileExpired,

    #[error("Error creating download record: {0}")]
    DownloadCreateError(sqlx::Error),

    #[error("Download URL not found")]
    DownloadNotFound,

    #[error("Download has expired")]
    DownloadExpired,

    #[error("Failed to open file")]
    FileOpenError,
}

impl Into<ApiError> for FileDownloadError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<Download> for ApiDownload {
    fn from(download: Download) -> Self {
        Self {
            id: download.id,
            expires_at: download.expires_at,
        }
    }
}

pub async fn handle_file_request_download(
    State(state): State<AppState>,
    Path(file_id): Path<Uuid>,
    user: Option<User>,
) -> Response {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &file_id).await else {
        let mut api_error: ApiError = FileDownloadError::FileNotFound.into();
        api_error.status = StatusCode::NOT_FOUND;
        return api_error.into_response();
    };

    if file.is_expired() {
        let mut api_error: ApiError = FileDownloadError::FileExpired.into();
        api_error.status = StatusCode::BAD_REQUEST;
        return api_error.into_response();
    }

    let download = create_download(pool, &file.id, user.map(|u| u.id))
        .await
        .map(|download| ApiDownload::from(download));

    if let Err(e) = download {
        let api_error: ApiError = FileDownloadError::DownloadCreateError(e).into();
        return api_error.into_response();
    };

    let api_download: ApiDownload = download.unwrap().into();
    Json(api_download).into_response()
}

pub async fn handle_file_download(
    State(state): State<AppState>,
    Path(download_id): Path<Uuid>,
    user: Option<User>,
) -> Result<Response, StatusCode> {
    let pool = state.get_pool();

    let Ok(download) = get_download_by_id(pool, &download_id).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if download.is_expired() {
        return Err(StatusCode::NOT_FOUND);
    }

    let is_same_user = user.map(|u| u.id) == download.created_by_id;

    if !is_same_user {
        return Err(StatusCode::FORBIDDEN);
    }

    let Ok(file) = get_file_by_id(pool, &download.file_id).await else {
        return Err(StatusCode::NOT_FOUND);
    };

    let file_path = file.get_path();
    let Ok(io_file) = tokio::fs::File::open(file_path).await else {
        let _error = FileDownloadError::FileOpenError;
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let reader_stream = ReaderStream::new(io_file);
    let body = Body::from_stream(reader_stream);

    // Pretend that this would get a download URL link from S3 or Cloud Storage
    Ok(body.into_response())
}
