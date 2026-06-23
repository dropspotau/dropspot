use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::response::Response;
use axum::routing::{get, patch, post};
use http::Method;
use tokio::net::TcpListener;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing::{Span, info_span};

use crate::db::connect;

use crate::handlers::{
    handle_create_user, handle_delete_file, handle_file_download, handle_file_request_download,
    handle_file_request_upload, handle_file_upload, handle_get_file,
    handle_get_integration_by_slug, handle_get_integrations, handle_health, handle_list_files,
    handle_login, handle_preview_upload, handle_refresh_tokens, handle_root, handle_update_file,
    handle_upsert_integration,
};

#[cfg(feature = "web")]
use crate::handlers::{
    handle_files, handle_header, handle_index, handle_onboarding, handle_record_onboarding,
    handle_settings, handle_update_settings, handle_update_user,
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

fn get_cors_layer() -> CorsLayer {
    CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        // allow requests from any origin
        .allow_origin(Any)
}

pub fn get_api_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(handle_health))
        .route("/upload", post(handle_file_request_upload))
        .route("/upload/preview", get(handle_preview_upload))
        .route("/upload/{file_id}", post(handle_file_upload))
        .route("/file", get(handle_list_files))
        .route(
            "/file/{id}",
            get(handle_get_file)
                .patch(handle_update_file)
                .delete(handle_delete_file),
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
        .route(
            "/onboarding",
            get(handle_onboarding).post(handle_record_onboarding),
        )
}

pub async fn handle_run_server() -> Result<(), ()> {
    init_tracing();

    let Ok(pool) = connect().await else {
        return Err(());
    };

    let state = AppState::new(Arc::new(pool));
    let serve_dir =
        ServeDir::new("static").not_found_service(ServeFile::new("static/not_found.html"));

    let api_router = get_api_router();
    let web_router = get_web_router();
    let cors_layer = get_cors_layer();

    let app = Router::new()
        .route("/", get(handle_root))
        .nest("/api", api_router)
        .nest("/app", web_router)
        .nest_service("/static", serve_dir.clone())
        .layer(cors_layer)
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
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        tracing::error!(error = %_error, "request failed")
                    },
                ),
        )
        .fallback_service(serve_dir)
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>();

    let port = std::env::var("DROPSPOT_PORT")
        .map(|port| {
            port.parse::<i32>()
                .expect("Could not parse DROPSPOT_PORT environment variable")
        })
        .unwrap_or(8000);
    let address = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&address)
        .await
        .expect(&format!("Could not bind to {address}"));

    tracing::info!("Listening on {address}");

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Server run error: {e}");
    }

    Ok(())
}
