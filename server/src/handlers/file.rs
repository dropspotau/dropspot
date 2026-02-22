use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    response::{IntoResponse, Response},
};
use dropspot_core::file::File as ApiFile;
use reqwest::StatusCode;
use thiserror::Error;
use uuid::Uuid;

use crate::db::{File, get_file_by_id, get_files};
use crate::state::AppState;
use crate::types::ApiError;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Could not list files")]
    FileListError(sqlx::Error),

    #[error("Could not retrieve file")]
    FileRetrieveError(sqlx::Error),
}

impl Into<ApiError> for FileError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
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

pub async fn handle_list_files(State(state): State<Arc<AppState>>) -> Response {
    let pool = state.get_pool();
    let files = get_files(&pool).await;

    if let Err(e) = files {
        let api_error: ApiError = FileError::FileListError(e).into();
        return api_error.into_response();
    }

    let files = files
        .unwrap()
        .into_iter()
        .map(|file| ApiFile::from(file))
        .collect::<Vec<ApiFile>>();

    Json(files).into_response()
}

pub async fn handle_get_file(State(state): State<Arc<AppState>>, Path(id): Path<Uuid>) -> Response {
    let pool = state.get_pool();
    let file = get_file_by_id(&pool, &id)
        .await
        .map(|file| ApiFile::from(file));

    if let Err(e) = file {
        let api_error: ApiError = FileError::FileRetrieveError(e).into();
        return api_error.into_response();
    }

    Json(file.unwrap()).into_response()
}
