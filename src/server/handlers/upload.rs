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
pub enum FileUploadError {
    #[error("Upload not found")]
    UploadNotFound,

    #[error("Error writing file to database: {0}")]
    FileDatabaseCreateError(sqlx::Error),

    #[error("Error deleting file from database: {0}")]
    FileDatabaseDeleteError(sqlx::Error),

    #[error("Failed to create file")]
    FileCreateError,

    #[error("Failed to write to file")]
    FileWriteError,
}

impl Into<ApiError> for FileUploadError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiUpload {
    pub id: Uuid,
}

impl From<Upload> for ApiUpload {
    fn from(upload: Upload) -> Self {
        Self { id: upload.id }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUploadBody {
    pub name: String,
    pub size: usize,
}

// TODO(alec): Make this into an Axum view
pub async fn handle_file_request_upload(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiUpload>, Json<ApiError>> {
    let upload = create_upload(state.get_pool()).await.map(Into::into);

    if let Err(e) = upload {
        return Err(Json(FileUploadError::FileDatabaseCreateError(e).into()));
    }

    return Ok(Json(upload.unwrap()));
}

#[derive(Serialize, Deserialize)]
pub struct ApiFile {
    pub id: Uuid,
    pub name: String,
    pub size: i64,
}

impl From<File> for ApiFile {
    fn from(file: File) -> Self {
        Self {
            id: file.id,
            name: file.name,
            size: file.size,
        }
    }
}

#[derive(Deserialize)]
struct FileUploadPayload {
    file_name: String,
    contents: Vec<u8>,
}

// TODO(alec): Make this into an Axum view
pub async fn handle_file_upload(
    State(state): State<Arc<AppState>>,
    Path(upload_id): Path<Uuid>,
    Json(payload): Json<FileUploadPayload>,
) -> Result<Json<ApiFile>, Json<FileUploadError>> {
    let pool = state.get_pool();

    let Ok(upload) = get_upload_by_id(pool, &upload_id).await else {
        return Err(Json(FileUploadError::UploadNotFound));
    };

    let path = PathBuf::from(&payload.file_name);
    let size = payload.contents.len();

    let pool = state.get_pool();
    let file = create_file(
        pool,
        payload.file_name,
        &upload.id,
        path.to_str().unwrap().to_owned(),
        size as i64,
    )
    .await;

    if let Err(e) = file {
        return Err(Json(FileUploadError::FileDatabaseCreateError(e)));
    }

    let file = file.unwrap();

    let Ok(mut io_file) = std::fs::File::create(file.get_path()) else {
        return Err(Json(FileUploadError::FileCreateError));
    };

    if io_file.write(&payload.contents).is_err() {
        // Don't save the file
        if let Err(e) = delete_files(pool, &[file.id]).await {
            return Err(Json(FileUploadError::FileDatabaseCreateError(e)));
        };

        return Err(Json(FileUploadError::FileWriteError));
    }

    Ok(Json(file.into()))
}
