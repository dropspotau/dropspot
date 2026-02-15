use std::{path::PathBuf, sync::Arc};

use axum::{
    body::Body,
    extract::{Json, Path, State},
    response::{IntoResponse, Response},
};
use futures_util::StreamExt;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::io::{AsyncWriteExt, BufWriter};
use uuid::Uuid;

use super::super::{
    db::{File, Upload, create_file, create_upload, delete_files, get_upload_by_id},
    state::AppState,
    types::ApiError,
};

#[derive(Error, Debug)]
pub enum FileUploadError {
    #[error("Upload not found")]
    UploadNotFound,

    #[error("Error writing file to database: {0}")]
    FileDatabaseCreateError(sqlx::Error),

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
pub async fn handle_file_request_upload(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUploadBody>,
) -> Response {
    let upload = create_upload(state.get_pool(), payload.name)
        .await
        .map(|upload| ApiUpload::from(upload))
        .map_err(FileUploadError::FileDatabaseCreateError);

    if let Err(e) = upload {
        let api_error: ApiError = e.into();
        return api_error.into_response();
    }

    println!("Uploaded");
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

// TODO(alec): Make this into an Axum view
pub async fn handle_file_upload(
    State(state): State<Arc<AppState>>,
    Path(upload_id): Path<Uuid>,
    body: Body,
) -> Response {
    let pool = state.get_pool();

    let Ok(upload) = get_upload_by_id(pool, &upload_id).await else {
        let mut api_error: ApiError = FileUploadError::UploadNotFound.into();
        api_error.status = StatusCode::NOT_FOUND;
        return api_error.into_response();
    };

    let size: usize = 10; // TODO(alec): Get size from header
    let file_name = "test.txt".to_string();
    let path = PathBuf::from(&file_name);

    let pool = state.get_pool();
    let file = create_file(
        pool,
        file_name,
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
    let Ok(io_file) = tokio::fs::File::create(file.get_path()).await else {
        let api_error: ApiError = FileUploadError::FileCreateError.into();
        return api_error.into_response();
    };

    let mut reader_stream = body.into_data_stream();
    let mut writer = BufWriter::new(io_file);

    while let Some(bytes) = reader_stream.next().await {
        if bytes.is_err() {
            if let Err(e) = delete_files(pool, &[file.id]).await {
                let api_error: ApiError = FileUploadError::FileDatabaseCreateError(e).into();
                return api_error.into_response();
            };

            let api_error: ApiError = FileUploadError::FileWriteError.into();
            return api_error.into_response();
        };

        if let Err(e) = writer.write(&bytes.unwrap()).await {
            eprintln!("Error writing to file: {e}");

            if let Err(e) = delete_files(pool, &[file.id]).await {
                let api_error: ApiError = FileUploadError::FileDatabaseCreateError(e).into();
                return api_error.into_response();
            };

            let api_error: ApiError = FileUploadError::FileWriteError.into();
            return api_error.into_response();
        };
    }

    println!("Uploaded file {}", file.id);
    let api_file: ApiFile = file.into();
    Json(api_file).into_response()
}
