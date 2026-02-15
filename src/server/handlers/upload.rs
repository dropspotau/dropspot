use std::{
    io::{Read, Write},
    path::PathBuf,
    sync::Arc,
};

use axum::{
    extract::{Json, Path, State},
    response::{IntoResponse, Response},
};
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
            status: StatusCode::INTERNAL_SERVER_ERROR,
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
pub async fn handle_file_request_upload(State(state): State<Arc<AppState>>) -> Response {
    let upload = create_upload(state.get_pool())
        .await
        .map(|upload| ApiUpload::from(upload))
        .map_err(FileUploadError::FileDatabaseCreateError);

    if let Err(e) = upload {
        let api_error: ApiError = e.into();
        return api_error.into_response();
    }

    return Json(upload.unwrap()).into_response();
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
) -> Response {
    let pool = state.get_pool();

    let Ok(upload) = get_upload_by_id(pool, &upload_id).await else {
        let api_error: ApiError = FileUploadError::UploadNotFound.into();
        return api_error.into_response();
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
        let api_error: ApiError = FileUploadError::FileDatabaseCreateError(e).into();
        return api_error.into_response();
    }

    let file = file.unwrap();

    // TODO(alec): Create file providers to upload to AWS, GCP etc.
    let Ok(mut io_file) = std::fs::File::create(file.get_path()) else {
        let api_error: ApiError = FileUploadError::FileCreateError.into();
        return api_error.into_response();
    };

    if io_file.write(&payload.contents).is_err() {
        // Don't save the file
        if let Err(e) = delete_files(pool, &[file.id]).await {
            let api_error: ApiError = FileUploadError::FileDatabaseCreateError(e).into();
            return api_error.into_response();
        };

        let api_error: ApiError = FileUploadError::FileWriteError.into();
        return api_error.into_response();
    }

    let api_file: ApiFile = file.into();
    Json(api_file).into_response()
}
