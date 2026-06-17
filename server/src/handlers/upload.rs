use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{ConnectInfo, Json, Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use dropspot_core::upload::CreateFileBody;
use dropspot_core::{file::File as ApiFile, upload::PreviewUploadRequest};
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
    state::AppState,
    storage::{StorageType, get_storage},
    types::ApiError,
};

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
        return ApiError::new(
            format!("Failed to retrieve organisation: {e}"),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    }

    let Ok(settings) = get_organisation_settings(pool, &organisation.unwrap().id).await else {
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

    let expires_at = Utc::now() + Duration::minutes(settings.default_file_expiry_minutes as i64);
    let max_downloads = settings.default_download_limit;

    let file = create_file(
        pool,
        &payload.name,
        &payload.name,
        payload.size,
        user.map(|u| u.id),
        &StorageType::from(payload.storage),
        expires_at,
        max_downloads,
        upload_ip,
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
    let organisation = get_organisation_from_request_user(pool, user.as_ref()).await;

    let Ok(file) = get_file_by_id(pool, &file_id).await else {
        let api_error = ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    let mut reader_stream = body.into_data_stream();

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

pub async fn handle_preview_upload(
    State(state): State<AppState>,
    user: Option<User>,
    Query(_payload): Query<PreviewUploadRequest>,
) -> Response {
    let pool = state.get_pool();

    let Ok(organisation) = get_organisation_from_request_user(pool, user.as_ref()).await else {
        return ApiError::new(
            format!("Failed to retrieve organisation"),
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
