use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{ConnectInfo, Json, Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use dropspot::{
    file::File as ApiFile,
    upload::{CreateFileBody, PreviewUploadRequest},
};
use futures_util::StreamExt;
use reqwest::StatusCode;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    db::{
        User, create_file, delete_files, finish_upload, get_file_by_id, get_integration_by_slug,
        get_organisation_settings, get_upload_by_file_id, preview_upload, start_upload,
    },
    handlers::utils::{extract_client_ip, get_organisation_from_request_user},
    permissions::file::can_see_file,
    state::AppState,
    storage::{StorageType, get_storage},
    types::ApiError,
};

// The endpoint which is called to initiate a file upload. This returns an Upload instance which can
// then be used with the handle_file_upload endpoint to actually upload the file.
//
// This validates if the requesting user can actually upload a file.
pub async fn handle_file_request_upload(
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(state): State<AppState>,
    user: Option<User>,
    Json(payload): Json<CreateFileBody>,
) -> Response {
    let pool = state.get_pool();
    let organisation = get_organisation_from_request_user(pool, user.as_ref()).await;
    let upload_ip = extract_client_ip(address, headers);

    if let Err(e) = organisation {
        tracing::error!("Failed to retrieve organisation: {e}");
        return ApiError::new(
            "Failed to retrieve organisation:".to_owned(),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    }

    let organisation = organisation.unwrap();

    let Ok(settings) = get_organisation_settings(pool, &organisation.id).await else {
        return ApiError::new(
            "Failed to retrieve settings for organisation".to_owned(),
            StatusCode::NOT_FOUND,
        )
        .into_response();
    };

    let can_upload = settings.allow_external_uploads || user.is_some();

    if !can_upload {
        return ApiError::new(
            "This organisation requires authentication to upload".to_owned(),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    }

    // Unauthorised uploads can only be downloaded once and within five minutes
    let duration_minutes = match user.is_some() {
        true => settings.default_file_expiry_minutes as i64,
        false => 5,
    };
    let expires_at = Utc::now() + Duration::minutes(duration_minutes);
    let max_downloads = match user.is_some() {
        true => settings.default_download_limit,
        false => 1,
    };

    let file = create_file(
        pool,
        &payload.name,
        payload.size,
        user.map(|u| u.id),
        &StorageType::from(payload.storage),
        expires_at,
        max_downloads,
        upload_ip,
        &organisation.id,
    )
    .await
    .map(ApiFile::from);

    if file.is_err() {
        let api_error = ApiError::new("Failed to create file".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    }

    Json(file.unwrap()).into_response()
}

// The endpoint which actually handles uploading an encrypted file's contents.
pub async fn handle_file_upload(
    State(state): State<AppState>,
    user: Option<User>,
    Path(file_id): Path<Uuid>,
    body: Body,
) -> Response {
    let pool = state.get_pool();
    let organisation = get_organisation_from_request_user(pool, user.as_ref()).await;

    let Ok(file) = get_file_by_id(pool, &file_id).await else {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    };

    if !can_see_file(&file, user.as_ref()) {
        // NOTE(alec): Probably not that relevant to ceck
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    }

    let mut reader_stream = body.into_data_stream();

    if let Err(e) = organisation {
        return ApiError::new(
            format!("Failed to retrieve organisation: {e}"),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    }

    let organisation_id = organisation.unwrap().id;
    let Ok(integration) = get_integration_by_slug(pool, &organisation_id, &file.storage).await
    else {
        return ApiError::new(
            format!("Integration not found for organisation"),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    };

    let Ok(settings) = get_organisation_settings(pool, &organisation_id).await else {
        tracing::error!("Failed to retrieve setting for organisation on file upload");
        return ApiError::new(
            "Failed to retrieve settings for organisation".to_owned(),
            StatusCode::NOT_FOUND,
        )
        .into_response();
    };

    let storage = get_storage(&integration.data);

    let Ok(mut writer) = storage.get_upload_writer(&file).await else {
        tracing::error!("Failed to write file");
        return ApiError::new("Failed to write file ".to_owned(), StatusCode::NOT_FOUND)
            .into_response();
    };

    let Ok(upload) = get_upload_by_file_id(pool, &file.id).await else {
        tracing::error!("Upload file not found");
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    };

    let is_same_user = file.created_by_id == user.map(|u| u.id);

    if !is_same_user {
        tracing::error!("Different user tried to upload this file");
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    };

    if start_upload(pool, &upload.id).await.is_err() {
        let api_error = ApiError::new(
            "Failed to create file".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    };

    let mut total_bytes: i64 = 0;
    let maximum_file_size_bytes = (settings.max_file_size_mb * 1024 * 1024) as i64;

    while let Some(bytes) = reader_stream.next().await {
        if bytes.is_err() {
            if delete_files(pool, &[file.id]).await.is_err() {
                return ApiError::new(
                    "Failed to upload file".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
                .into_response();
            };

            return ApiError::new(
                "Failed to write file".to_owned(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        };

        // Keep track if the byte size is too large
        let bytes = bytes.unwrap();
        total_bytes += bytes.len() as i64;

        let has_exceeded_size = total_bytes > maximum_file_size_bytes;

        if has_exceeded_size {
            // TODO(alec): At the time of writing, all uploads write to a temporary directory before uploading
            // anything. When actual streamed uploads are implemented, we'll want to actually clear
            // up any half-uploaded files
            if let Err(e) = delete_files(pool, &[file.id]).await {
                tracing::error!("Failed to delete uploading file while size is too large: {e}");
            };

            return ApiError::new(
                "Maximum file size exceeded".to_owned(),
                StatusCode::PAYLOAD_TOO_LARGE,
            )
            .into_response();
        }

        if let Err(e) = writer.write(&bytes).await {
            tracing::error!("Error writing to file: {e}");

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
            tracing::error!("Error flushing to file: {e}");

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
        tracing::error!("Error finishing upload: {e:?}");

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

pub async fn handle_preview_upload(
    State(state): State<AppState>,
    user: Option<User>,
    Query(payload): Query<PreviewUploadRequest>,
) -> Response {
    let pool = state.get_pool();

    let Ok(organisation) = get_organisation_from_request_user(pool, user.as_ref()).await else {
        return ApiError::new(
            "Failed to retrieve organisation".to_owned(),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    };

    let Ok(settings) = get_organisation_settings(pool, &organisation.id).await else {
        return ApiError::new(
            "Failed to retrieve settings for organisation".to_owned(),
            StatusCode::NOT_FOUND,
        )
        .into_response();
    };

    let can_upload = settings.allow_external_uploads || user.is_some();

    if !can_upload {
        return ApiError::new(
            "This organisation requires authentication to upload".to_owned(),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    }

    let file_size_mb = (payload.size / 1024 / 1024) as i32;
    let is_too_large = file_size_mb > settings.max_file_size_mb;

    if is_too_large {
        return ApiError::new("File is too large".to_owned(), StatusCode::BAD_REQUEST)
            .into_response();
    }

    let upload_preiew = preview_upload(pool, &organisation.id).await;

    if let Err(e) = upload_preiew {
        return ApiError::new(
            format!("Failed to determine upload preview: {e}"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    }

    Json(upload_preiew.unwrap()).into_response()
}
