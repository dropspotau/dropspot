use axum::{
    extract::{Json, Path, State},
    response::{IntoResponse, Response},
};
use chrono::Utc;
use dropspot::file::{File as ApiFile, UpdateFilePayload};
use reqwest::StatusCode;
use uuid::Uuid;

use crate::{
    db::get_files_by_uploader_id,
    permissions::file::{can_delete_file, can_see_file, can_update_file},
    types::ApiError,
};
use crate::{db::update_file, state::AppState};
use crate::{
    db::{
        File, User, delete_files, get_file_by_id, get_files, get_integration_by_slug,
        get_organisation_for_user,
    },
    storage::get_storage,
};

impl From<File> for ApiFile {
    fn from(file: File) -> Self {
        Self {
            id: file.id,
            name: file.name.clone(),
            size: file.size,
            expires_at: file.expires_at.to_rfc3339(),
            remaining_downloads: file.get_remaining_downloads(),
            max_downloads: file.max_downloads,
            is_expired: file.is_expired(),
            mime_type: file.get_mime_type().to_owned(),
        }
    }
}

pub async fn handle_list_files(State(state): State<AppState>, user: User) -> Response {
    let pool = state.get_pool();
    let files = match user.is_admin {
        true => get_files(pool).await,
        false => get_files_by_uploader_id(pool, &user.id).await,
    };
    let Ok(files) = files else {
        return ApiError::new(
            "Could not list files".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };

    let files = files
        .into_iter()
        .filter(|file| can_see_file(file, Some(&user)))
        .map(|file| ApiFile::from(file))
        .collect::<Vec<ApiFile>>();

    Json(files).into_response()
}

pub async fn handle_get_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    user: Option<User>,
) -> Response {
    let pool = state.get_pool();
    let Ok(file) = get_file_by_id(&pool, &id).await else {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    };

    if !can_see_file(&file, user.as_ref()) {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    }

    Json(ApiFile::from(file)).into_response()
}

pub async fn handle_update_file(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    user: User,
    Json(payload): Json<UpdateFilePayload>,
) -> Response {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &id).await else {
        let api_error = ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    if !can_see_file(&file, Some(&user)) {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    }

    if !can_update_file(&file, &user) {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    }

    let now = Utc::now();
    let expires_at = payload.expires_at.unwrap_or(file.expires_at);
    let max_downloads = payload.max_downloads.unwrap_or(file.max_downloads);

    if expires_at <= now {
        return ApiError::new(
            "File expiry cannot be in the past".to_owned(),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
    }

    if max_downloads <= 0 {
        return ApiError::new(
            "Maximum downloads must be positive".to_owned(),
            StatusCode::BAD_REQUEST,
        )
        .into_response();
    }

    let Ok(file) = update_file(pool, &id, expires_at, max_downloads).await else {
        return ApiError::new("Failed to update file".to_owned(), StatusCode::BAD_REQUEST)
            .into_response();
    };

    Json(ApiFile::from(file)).into_response()
}

pub async fn handle_delete_file(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Response {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &id).await else {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    };

    if !can_see_file(&file, Some(&user)) {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    }

    if !can_delete_file(&file, &user) {
        // Only uploaders can delete their files
        return ApiError::new(
            "Cannot delete another's file".to_owned(),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    }

    if let Err(_e) = delete_files(pool, &[file.id]).await {
        return ApiError::new(
            "Failed to delete file".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
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
