use axum::{
    extract::{Json, Path, State},
    response::{IntoResponse, Response},
};
use dropspot_core::file::File as ApiFile;
use reqwest::StatusCode;
use thiserror::Error;
use uuid::Uuid;

use crate::state::AppState;
use crate::types::ApiError;
use crate::{
    db::{
        File, User, delete_files, get_file_by_id, get_files, get_integration_by_slug,
        get_organisation_for_user,
    },
    storage::get_storage,
};

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
            name: file.name.clone(),
            size: file.size,
            remaining_downloads: file.get_remaining_downloads(),
        }
    }
}

pub async fn handle_list_files(State(state): State<AppState>) -> Response {
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

pub async fn handle_get_file(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
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

pub async fn handle_delete_file(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Response {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &id).await else {
        let api_error = ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    if let Some(ref user_id) = file.created_by_id
        && user_id != &user.id
    {
        // Only uploaders can delete their files
        let api_error = ApiError::new("Cannot delete another's file".to_owned(), StatusCode::UNAUTHORIZED);
        return api_error.into_response();
    }

    if let Err(_e) = delete_files(pool, &[file.id]).await {
        let api_error = ApiError::new(
            "Failed to delete file".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    }

    let organisation = get_organisation_for_user(pool, &user.id).await;

    if let Err(e) = organisation {
        return ApiError::new(
            format!("Failed to retrieve organisation: {e}"),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    }

    let organisation = Some(organisation.unwrap());
    let Ok(integration) =
        get_integration_by_slug(pool, &organisation.unwrap().id, &file.storage).await
    else {
        return ApiError::new(
            format!("Integration not found for organisation"),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    };

    let storage = get_storage(&integration.data);

    if storage.delete(&file).await.is_err() {
        eprintln!("Failed to delete file: {}", file.id);
    }

    ApiError::new("".to_owned(), StatusCode::NO_CONTENT).into_response()
}
