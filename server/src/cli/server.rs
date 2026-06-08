use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, patch, post};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

use crate::db::connect;

use crate::handlers::{
    handle_create_user, handle_delete_file, handle_file_download, handle_file_request_download,
    handle_file_request_upload, handle_file_upload, handle_files, handle_get_file,
    handle_get_integration_by_slug, handle_get_integrations, handle_header, handle_index,
    handle_list_files, handle_login, handle_preview_upload, handle_refresh_tokens, handle_settings,
    handle_update_settings, handle_update_user, handle_upsert_integration,
};
use crate::state::AppState;
use crate::watch::watch_for_files;

pub async fn handle_watch() -> Result<(), ()> {
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
        .route("/upload/{file_id}/upload", post(handle_file_upload))
        .route("/file", get(handle_list_files))
        .route("/file/{id}", get(handle_get_file))
        .route("/file/{id}/delete", delete(handle_delete_file))
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
        .fallback_service(serve_dir)
        .with_state(state);

    println!("Listening on port 8000");
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server run error: {e}");
    }

    Ok(())
}
