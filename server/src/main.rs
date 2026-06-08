mod auth;
mod cli;
mod db;
mod handlers;
mod middleware;
mod state;
mod storage;
mod types;
mod watch;

use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, patch, post};
use clap::{Parser, Subcommand};
use dropspot_core::auth::Authentication;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;

use crate::auth::storage::get_access_token;
use crate::cli::{
    auth::{handle_create_user as handle_cli_create_user, handle_login as handle_cli_login},
    file::{
        handle_download as handle_cli_download, handle_get_file as handle_cli_get_file,
        handle_list_files as handle_cli_list_files, handle_upload as handle_cli_upload,
    },
};
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
use dropspot_core::file::{get_file, list_files};

#[derive(Parser)]
#[command(name = "dropspot")]
#[command(about = "A simple file sharing CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum FileCommands {
    #[command(about = "Upload a file")]
    Upload { file: String },
    #[command(about = "Download a file")]
    Download {
        id: Uuid,
        key: String,
        nonce: String,
    },
    #[command(about = "List files")]
    List,
    #[command(about = "Retrieve a file")]
    Get { id: Uuid },
}

#[derive(Subcommand)]
enum ServerCommands {
    #[command(about = "Watch for files")]
    Watch,
    #[command(about = "Run the server")]
    Run,
}

#[derive(Subcommand)]
enum AuthCommands {
    #[command(about = "Log into DropSpot")]
    Login,
    #[command(about = "Create a user")]
    Create,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    File(FileCommands),

    #[command(subcommand)]
    Server(ServerCommands),

    #[command(subcommand)]
    Auth(AuthCommands),
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::File(file_commands) => match file_commands {
            FileCommands::Upload { file: file_name } => return handle_cli_upload(file_name).await,
            FileCommands::Download { id, key, nonce } => {
                return handle_cli_download(id, key, nonce).await;
            }
            FileCommands::List {} => return handle_cli_list_files().await,
            FileCommands::Get { id } => return handle_cli_get_file(id).await,
        },
        Commands::Server(server_commands) => match server_commands {
            ServerCommands::Watch {} => {
                let Ok(pool) = connect().await else {
                    return Err(());
                };

                let state = AppState::new(Arc::new(pool));
                watch_for_files(state).await;
            }
            ServerCommands::Run => {
                let Ok(pool) = connect().await else {
                    return Err(());
                };

                let state = AppState::new(Arc::new(pool));
                let serve_dir = ServeDir::new("static")
                    .not_found_service(ServeFile::new("static/not_found.html"));

                let app = Router::new()
                    .route("/api/upload", post(handle_file_request_upload))
                    .route("/api/upload/preview", get(handle_preview_upload))
                    .route("/api/upload/{file_id}/upload", post(handle_file_upload))
                    .route("/api/file", get(handle_list_files))
                    .route("/api/file/{id}", get(handle_get_file))
                    .route("/api/file/{id}/delete", delete(handle_delete_file))
                    .route(
                        "/api/file/{file_id}/download",
                        get(handle_file_request_download),
                    )
                    .route(
                        "/api/download/{download_id}/download",
                        get(handle_file_download),
                    )
                    .route("/api/user/login", post(handle_login))
                    .route("/api/user/create", post(handle_create_user))
                    .route("/api/user/refresh", post(handle_refresh_tokens))
                    .route("/api/integrations", get(handle_get_integrations))
                    .route(
                        "/api/integrations/{slug}",
                        get(handle_get_integration_by_slug),
                    )
                    .route(
                        "/api/integrations/{slug}/upsert",
                        patch(handle_upsert_integration),
                    )
                    .route("/app", get(handle_index))
                    .route("/app/header", get(handle_header))
                    .route("/app/files", get(handle_files))
                    .route("/app/settings", get(handle_settings))
                    .route("/app/settings/update", patch(handle_update_settings))
                    .route("/app/settings/user/{id}/update", patch(handle_update_user))
                    .nest_service("/static", serve_dir.clone())
                    .fallback_service(serve_dir)
                    .with_state(state);

                println!("Listening on port 8000");
                let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
                if let Err(e) = axum::serve(listener, app).await {
                    eprintln!("Server run error: {e}");
                }
            }
        },
        Commands::Auth(auth_commands) => match auth_commands {
            AuthCommands::Login {} => return handle_cli_login().await,
            AuthCommands::Create {} => return handle_cli_create_user().await,
        },
    }

    Ok(())
}
