use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::response::Response;
use axum::routing::{get, patch, post};
use bytes::Bytes;
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{Span, info_span};

use crate::db::connect;

use crate::handlers::{
    handle_create_user, handle_delete_file, handle_file_download, handle_file_request_download,
    handle_file_request_upload, handle_file_upload, handle_files, handle_get_file,
    handle_get_integration_by_slug, handle_get_integrations, handle_list_files, handle_login,
    handle_preview_upload, handle_refresh_tokens, handle_upsert_integration,
};

#[cfg(feature = "web")]
use crate::handlers::{
    handle_header, handle_index, handle_settings, handle_update_settings, handle_update_user,
};

use crate::state::AppState;
use crate::tracing::init_tracing;
use crate::watch::watch_for_files;

pub async fn handle_watch() -> Result<(), ()> {
    init_tracing();

    let Ok(pool) = connect().await else {
        return Err(());
    };

    let state = AppState::new(Arc::new(pool));
    watch_for_files(state).await;

    Ok(())
}

pub fn get_api_router() -> Router<AppState> {
    Router::new()
        .route("/upload", post(handle_file_request_upload))
        .route("/upload/preview", get(handle_preview_upload))
        .route("/upload/{file_id}", post(handle_file_upload))
        .route("/file", get(handle_list_files))
        .route(
            "/file/{id}",
            get(handle_get_file).delete(handle_delete_file),
        ) // TODO(alec): Update file
        .route(
            "/file/{file_id}/download",
            get(handle_file_request_download),
        )
        .route(
            "/download/{download_id}/download",
            get(handle_file_download),
        )
        .route("/user/login", post(handle_login))
        .route("/user/create", post(handle_create_user))
        .route("/user/refresh", post(handle_refresh_tokens))
        .route("/integrations", get(handle_get_integrations))
        .route("/integrations/{slug}", get(handle_get_integration_by_slug))
        .route(
            "/integrations/{slug}/upsert",
            patch(handle_upsert_integration),
        )
}

#[cfg(not(feature = "web"))]
pub fn get_web_router() -> Router<AppState> {
    Router::new()
}

#[cfg(feature = "web")]
pub fn get_web_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_index))
        .route("/header", get(handle_header))
        .route("/files", get(handle_files))
        .route("/settings", get(handle_settings))
        .route("/settings/update", patch(handle_update_settings))
        .route("/settings/user/{id}/update", patch(handle_update_user))
}

pub async fn handle_run_server() -> Result<(), ()> {
    let Ok(pool) = connect().await else {
        return Err(());
    };

    let state = AppState::new(Arc::new(pool));
    let serve_dir =
        ServeDir::new("static").not_found_service(ServeFile::new("static/not_found.html"));

    let api_router = get_api_router();
    let web_router = get_web_router();

    let app = Router::new()
        .nest("/api", api_router)
        .nest("/app", web_router)
        .nest_service("/static", serve_dir.clone())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                    tracing::debug!("started processing request")
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    tracing::debug!("finished processing request")
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    tracing::debug!("sending body chunk")
                })
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!("something went wrong")
                    },
                ),
        )
        .fallback_service(serve_dir)
        .with_state(state);

    tracing::info!("Listening on port 8000");
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server run error: {e}");
    }

    Ok(())
}
