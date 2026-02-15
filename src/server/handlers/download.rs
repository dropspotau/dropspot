use std::{
    io::{Read, Write},
    path::PathBuf,
    sync::Arc,
};

use axum::extract::{Json, Path, State};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use super::super::{
    db::{
        Download, File, Upload, create_download, create_file, create_upload, delete_files,
        get_download_by_id, get_file_by_id, get_upload_by_id,
    },
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
) -> Result<Json<ApiDownload>, Json<FileDownloadError>> {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &file_id).await else {
        return Err(Json(FileDownloadError::FileNotFound));
    };

    if file.is_expired() {
        return Err(Json(FileDownloadError::FileExpired));
    }

    let download = create_download(pool, &file.id).await;

    if let Err(e) = download {
        return Err(Json(FileDownloadError::DownloadCreateError(e)));
    };

    Ok(Json(download.unwrap().into()))
}

// TODO(alec): Make this into an Axum view
pub async fn handle_file_download(
    State(state): State<Arc<AppState>>,
    Path(download_id): Path<Uuid>,
) -> Result<impl Iterator<Item = u8> + use<>, StatusCode> {
    let pool = state.get_pool();

    let Ok(download) = get_download_by_id(pool, &download_id).await else {
        eprintln!(
            "Download not found: {:?}",
            FileDownloadError::DownloadNotFound
        );
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    if download.is_expired() {
        // return Err(FileDownloadError::DownloadExpired);
        eprintln!(
            "Download not found: {:?}",
            FileDownloadError::DownloadExpired
        );
        return Err(StatusCode::NOT_FOUND);
    }

    let Ok(file) = get_file_by_id(pool, &download.file_id).await else {
        // return Err(FileDownloadError::FileNotFound);
        return Err(StatusCode::NOT_FOUND);
    };

    let file_path = file.get_path();
    let Ok(mut io_file) = std::fs::File::open(file_path) else {
        // return Err(FileDownloadError::FileOpenError);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    // TODO (alec): Don't read the whole file into memory
    let mut buffer = Vec::with_capacity(file.size as usize);
    if io_file.read_to_end(&mut buffer).is_err() {
        // return Err(FileDownloadError::FileReadError);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Pretend that this would get a download URL link from S3 or Cloud Storage
    Ok(buffer.into_iter())
}
