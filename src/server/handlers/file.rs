use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::server::{
    ApiError,
    db::{File, get_files},
};

use super::super::state::AppState;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Could not list files")]
    FileListError(sqlx::Error),
}

impl Into<ApiError> for FileError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
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
