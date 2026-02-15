use std::{io::Read, sync::Arc};

use axum::{
    body::Body,
    extract::{Json, Path, State},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use super::super::{
    db::{Download, create_download, get_download_by_id, get_file_by_id},
    state::AppState,
    types::ApiError,
};

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

    #[error("Failed to read file")]
    FileReadError,
}

impl Into<ApiError> for FileDownloadError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct ApiDownload {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
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
    State(state): State<Arc<AppState>>,
    Path(file_id): Path<Uuid>,
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

    let download = create_download(pool, &file.id)
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
    State(state): State<Arc<AppState>>,
    Path(download_id): Path<Uuid>,
) -> Result<Response, StatusCode> {
    let pool = state.get_pool();

    let Ok(download) = get_download_by_id(pool, &download_id).await else {
        eprintln!(
            "Download not found: {:?}",
            FileDownloadError::DownloadNotFound
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if download.is_expired() {
        return Err(StatusCode::NOT_FOUND);
    }

    let Ok(file) = get_file_by_id(pool, &download.file_id).await else {
        return Err(StatusCode::NOT_FOUND);
    };

    let file_path = file.get_path();
    let Ok(io_file) = tokio::fs::File::open(file_path).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let reader_stream = ReaderStream::new(io_file);

    let body = Body::from_stream(reader_stream);

    // Pretend that this would get a download URL link from S3 or Cloud Storage
    Ok(body.into_response())
}
