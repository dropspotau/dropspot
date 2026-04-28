use axum::{
    body::Body,
    extract::{Json, Path, Query, State},
    response::{IntoResponse, Response},
};
use dropspot_core::upload::CreateFileBody;
use dropspot_core::{file::File as ApiFile, upload::CanUploadRequest};
use futures_util::StreamExt;
use reqwest::StatusCode;
use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    db::{
        User, can_upload, create_file, delete_files, finish_upload, get_file_by_id,
        get_organisation_for_user, get_upload_by_file_id, start_upload,
    },
    state::AppState,
    storage::{StorageType, get_storage},
    types::ApiError,
};

pub async fn handle_file_request_upload(
    State(state): State<AppState>,
    user: Option<User>,
    Json(payload): Json<CreateFileBody>,
) -> Response {
    let file = create_file(
        state.get_pool(),
        &payload.name,
        &payload.name,
        payload.size,
        user.map(|u| u.id),
        &StorageType::from(payload.storage),
    )
    .await
    .map(ApiFile::from);

    if file.is_err() {
        let api_error = ApiError::new("Failed to create file".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    }

    Json(file.unwrap()).into_response()
}

pub async fn handle_file_upload(
    State(state): State<AppState>,
    user: Option<User>,
    Path(file_id): Path<Uuid>,
    body: Body,
) -> Response {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &file_id).await else {
        let api_error = ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    let mut reader_stream = body.into_data_stream();

    // TODO(alec): Create file providers to upload to AWS, GCP etc.
    let storage = get_storage(&file.storage);

    let Ok(mut writer) = storage.get_upload_writer(&file).await else {
        let api_error = ApiError::new("Failed to write file ".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    let Ok(upload) = get_upload_by_file_id(pool, &file.id).await else {
        let api_error = ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    let is_same_user = file.created_by_id == user.map(|u| u.id);

    if !is_same_user {
        let api_error = ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    if start_upload(pool, &upload.id).await.is_err() {
        let api_error = ApiError::new(
            "Failed to create file".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    };

    while let Some(bytes) = reader_stream.next().await {
        if bytes.is_err() {
            if delete_files(pool, &[file.id]).await.is_err() {
                let api_error = ApiError::new(
                    "Failed to upload file".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
                return api_error.into_response();
            };

            let api_error = ApiError::new(
                "Failed to write file".to_owned(),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            return api_error.into_response();
        };

        if let Err(e) = writer.write(&bytes.unwrap()).await {
            eprintln!("Error writing to file: {e}");

            if delete_files(pool, &[file.id]).await.is_err() {
                let api_error = ApiError::new(
                    "Failed to upload file".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
                return api_error.into_response();
            };

            let api_error = ApiError::new(
                "Failed to upload file".to_owned(),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            return api_error.into_response();
        };

        if let Err(e) = writer.flush().await {
            eprintln!("Error flushing to file: {e}");

            if delete_files(pool, &[file.id]).await.is_err() {
                let api_error = ApiError::new(
                    "Failed to upload file".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                );
                return api_error.into_response();
            };

            let api_error = ApiError::new(
                "Failed to write file".to_owned(),
                StatusCode::INTERNAL_SERVER_ERROR,
            );
            return api_error.into_response();
        }
    }

    if let Err(e) = storage.finish_upload(&file).await {
        eprintln!("Error finishing upload: {e:?}");
        let api_error = ApiError::new(
            "Failed to upload file".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    };

    if finish_upload(pool, &upload.id).await.is_err() {
        let api_error = ApiError::new(
            "Failed to upload file".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    };

    let api_file: ApiFile = file.clone().into();
    Json(api_file).into_response()
}

pub async fn handle_can_upload(
    State(state): State<AppState>,
    user: Option<User>,
    Query(payload): Query<CanUploadRequest>,
) -> Response {
    let pool = state.get_pool();

    let user = user.map(|u| u.id);
    let can_upload = can_upload(pool, user.as_ref(), payload.size).await;

    if let Err(e) = can_upload {
        return ApiError::new(
            format!("Failed to determine upload status: {e}"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    }

    Json(can_upload.unwrap()).into_response()
}
