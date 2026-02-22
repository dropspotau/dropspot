use std::sync::Arc;

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

use super::{
    super::{
        db::{
            create_file, delete_files, finish_upload, get_file_by_id, get_upload_by_file_id,
            start_upload,
        },
        state::AppState,
        types::ApiError,
    },
    file::ApiFile,
};

#[derive(Error, Debug)]
pub enum FileUploadError {
    #[error("Upload not found")]
    UploadNotFound,

    #[error("Could not record upload changes")]
    UploadDatabaseError(sqlx::Error),

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

pub async fn handle_file_request_upload(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateFileBody>,
) -> Response {
    let file = create_file(state.get_pool(), &payload.name, &payload.name, payload.size)
        .await
        .map(ApiFile::from)
        .map_err(FileUploadError::FileDatabaseCreateError);

    if let Err(e) = file {
        let api_error: ApiError = e.into();
        return api_error.into_response();
    }

    return Json(file.unwrap()).into_response();
}

pub async fn handle_file_upload(
    State(state): State<Arc<AppState>>,
    Path(file_id): Path<Uuid>,
    body: Body,
) -> Response {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &file_id).await else {
        let mut api_error: ApiError = FileUploadError::UploadNotFound.into();
        api_error.status = StatusCode::NOT_FOUND;
        return api_error.into_response();
    };

    // TODO(alec): Create file providers to upload to AWS, GCP etc.
    let Ok(io_file) = tokio::fs::File::create(file.get_path()).await else {
        let api_error: ApiError = FileUploadError::FileCreateError.into();
        return api_error.into_response();
    };

    let mut reader_stream = body.into_data_stream();
    let mut writer = BufWriter::new(io_file);

    let Ok(upload) = get_upload_by_file_id(pool, &file.id).await else {
        let api_error: ApiError = FileUploadError::UploadNotFound.into();
        return api_error.into_response();
    };

    if let Err(e) = start_upload(pool, &upload.id).await {
        let api_error: ApiError = FileUploadError::UploadDatabaseError(e).into();
        return api_error.into_response();
    };

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

        if let Err(e) = writer.flush().await {
            eprintln!("Error flushing to file: {e}");

            if let Err(e) = delete_files(pool, &[file.id]).await {
                let api_error: ApiError = FileUploadError::FileDatabaseCreateError(e).into();
                return api_error.into_response();
            };

            let api_error: ApiError = FileUploadError::FileWriteError.into();
            return api_error.into_response();
        }
    }

    if let Err(e) = finish_upload(pool, &upload.id).await {
        let api_error: ApiError = FileUploadError::UploadDatabaseError(e).into();
        return api_error.into_response();
    };

    let api_file: ApiFile = file.into();
    Json(api_file).into_response()
}
