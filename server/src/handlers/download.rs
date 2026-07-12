use std::net::SocketAddr;

use axum::{
    body::Body,
    extract::{ConnectInfo, Json, Path, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use dropspot::download::Download as ApiDownload;
use reqwest::StatusCode;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    db::{
        Download, User, create_download, get_download_by_id, get_file_by_id,
        get_integration_by_slug,
    },
    handlers::utils::{extract_client_ip, get_organisation_from_request_user},
    permissions::file::can_see_file,
};
use crate::{state::AppState, storage::get_storage, types::ApiError};

impl From<Download> for ApiDownload {
    fn from(download: Download) -> Self {
        Self {
            id: download.id,
            expires_at: download.expires_at,
        }
    }
}

pub async fn handle_file_request_download(
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Path(file_id): Path<Uuid>,
    user: Option<User>,
) -> Response {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &file_id).await else {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    };

    if !can_see_file(&file, user.as_ref()) {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    }

    let Ok(organisation) = get_organisation_from_request_user(pool, user.as_ref()).await else {
        tracing::error!("FAILED TO LOAD ORGanISTION");
        return ApiError::new(
            "Failed to retrieve organisation".to_owned(),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    };

    if organisation.id != file.organisation_id {
        tracing::error!(
            "Download organisation mismatch: {} - {} vs {}",
            file.id,
            organisation.id,
            file.organisation_id
        );
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
    }

    if file.is_expired() {
        let api_error = ApiError::new("File is expired".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    }

    let download_ip = extract_client_ip(address, headers);
    let download = create_download(pool, &file.id, user.map(|u| u.id), download_ip)
        .await
        .map(|download| ApiDownload::from(download));

    if download.is_err() {
        let api_error = ApiError::new(
            "Failed to record download".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    };

    let api_download: ApiDownload = download.unwrap().into();
    Json(api_download).into_response()
}

pub async fn handle_file_download(
    State(state): State<AppState>,
    Path(download_id): Path<Uuid>,
    user: Option<User>,
) -> Response {
    let pool = state.get_pool();
    let server_config = state.get_server_config();
    let organisation = get_organisation_from_request_user(pool, user.as_ref()).await;

    let Ok(download) = get_download_by_id(pool, &download_id).await else {
        let api_error = ApiError::new("Download not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    if download.is_expired() {
        let api_error = ApiError::new("Download is expired".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    }

    // Downloads can only be actioned by the same requesting user, ignored if it's unauthorised
    let can_download = user.is_none() || user.as_ref().map(|u| u.id) == download.created_by_id;

    if !can_download {
        let api_error = ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    }

    let Ok(file) = get_file_by_id(pool, &download.file_id).await else {
        let api_error = ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND);
        return api_error.into_response();
    };

    if let Err(e) = organisation {
        return ApiError::new(
            format!("Failed to retrieve organisation: {e}"),
            StatusCode::UNAUTHORIZED,
        )
        .into_response();
    }

    if !can_see_file(&file, user.as_ref()) {
        return ApiError::new("File not found".to_owned(), StatusCode::NOT_FOUND).into_response();
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

    let storage = get_storage(&integration.data, server_config);

    let Ok(reader) = storage.get_download_reader(&file).await else {
        let api_error = ApiError::new(
            "Failed to read file".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    };

    let reader_stream = ReaderStream::new(reader);
    let body = Body::from_stream(reader_stream);

    // Pretend that this would get a download URL link from S3 or Cloud Storage
    body.into_response()
}
