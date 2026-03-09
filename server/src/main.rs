mod auth;
mod db;
mod handlers;
mod middleware;
mod state;
mod types;
mod watch;

use std::fs::File;
use std::io::{BufWriter, Read};
use std::sync::Arc;

use axum::Router;
use axum::routing::{delete, get, post};
use base64::alphabet::URL_SAFE;
use base64::engine::GeneralPurpose;
use base64::engine::general_purpose::NO_PAD;
use base64::prelude::*;
use clap::{Parser, Subcommand};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use uuid::Uuid;

use crate::db::connect;
use crate::handlers::{
    handle_create_user, handle_delete_file, handle_file_download, handle_file_request_download,
    handle_file_request_upload, handle_file_upload, handle_files, handle_get_file, handle_header,
    handle_index, handle_list_files, handle_login, handle_settings,
};
use crate::state::AppState;
use crate::watch::watch_for_files;
use dropspot_core::encryption::Encryption;
use dropspot_core::{
    download::download,
    file::{get_file, list_files},
    upload::upload,
    validation::validate_file,
};

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
        id: String,
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
enum Commands {
    #[command(subcommand)]
    File(FileCommands),

    #[command(subcommand)]
    Server(ServerCommands),
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::File(file_commands) => match file_commands {
            FileCommands::Upload { file: file_name } => {
                // Simulate generating an upload URL
                let validation = validate_file(file_name);

                if let Err(e) = validation {
                    eprintln!("Failed to validate file: {e:?}");
                    return Err(());
                }

                let mut file = validation.unwrap();
                let mut buffer = Vec::new();
                if let Err(e) = file.read_to_end(&mut buffer) {
                    eprintln!("Failed to upload file: {e:?}");
                    return Err(());
                }

                let upload = upload(file_name.clone(), buffer).await;

                if let Err(e) = upload {
                    eprintln!("Failed to upload file: {e:?}");
                    return Err(());
                }

                let upload = upload.unwrap();

                let engine = GeneralPurpose::new(&URL_SAFE, NO_PAD);
                let key_base64 = engine.encode(&upload.encryption.key);
                let nonce_base64 = engine.encode(&upload.encryption.nonce);

                println!("Uploaded file {}", &upload.file.id);
                println!("Key: {key_base64}");
                println!("Key: {:?}", upload.encryption.key);
                println!("Nonce: {nonce_base64}");
                println!("Nonce: {:?}", upload.encryption.nonce);

                println!(
                    "cargo run file download {} {key_base64} {nonce_base64}",
                    upload.file.id
                );
            }
            FileCommands::Download { id, key, nonce } => {
                let Ok(id) = Uuid::parse_str(id) else {
                    eprintln!("Invalid UUID");
                    return Err(());
                };

                let engine = GeneralPurpose::new(&URL_SAFE, NO_PAD);
                let key = engine.decode(key).unwrap();
                let nonce = engine.decode(nonce).unwrap();

                let encryption = Encryption { key, nonce };

                let Ok(file) = get_file(&id).await else {
                    eprintln!("Failed to retrieve file details");
                    return Err(());
                };

                // Decrypt the file
                let local_file_name = format!("download_{}", &file.name);
                let Ok(local_file) = File::create(&local_file_name) else {
                    eprintln!("Failed to open local file to save");
                    return Err(());
                };

                let stream_writer = BufWriter::new(local_file);
                if let Err(e) = download(id, &encryption, stream_writer).await {
                    eprintln!("Failed to download file: {e}");
                    return Err(());
                }

                println!("Download complete");
            }
            FileCommands::List {} => {
                let files = list_files().await;

                if let Err(e) = files {
                    eprintln!("Failed to list files: {e}");
                    return Err(());
                }

                let files = files.unwrap();
                println!("{files:?}");
            }
            FileCommands::Get { id } => {
                let file = get_file(id).await;

                if let Err(e) = file {
                    eprintln!("Failed to get file: {e}");
                    return Err(());
                }

                let file = file.unwrap();
                println!("{file:?}");
            }
        },
        Commands::Server(server_commands) => match server_commands {
            ServerCommands::Watch {} => {
                let Ok(pool) = connect().await else {
                    return Err(());
                };

                let state = AppState::new(pool);
                watch_for_files(state).await;
            }
            ServerCommands::Run => {
                let Ok(pool) = connect().await else {
                    return Err(());
                };

                let state = AppState::new(pool);
                let shared_state = Arc::new(state);
                let serve_dir = ServeDir::new("static")
                    .not_found_service(ServeFile::new("static/not_found.html"));

                let app = Router::new()
                    .route("/api/upload", post(handle_file_request_upload))
                    .route("/api/upload/{file_id}/upload", post(handle_file_upload))
                    .route("/api/file", get(handle_list_files))
                    .route("/api/file/{id}", get(handle_get_file))
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
                    .route("/app", get(handle_index))
                    .route("/app/header", get(handle_header))
                    .route("/app/files", get(handle_files))
                    .route("/app/files/{id}/delete", delete(handle_delete_file))
                    .route("/app/settings", get(handle_settings))
                    .nest_service("/static", serve_dir.clone())
                    .fallback_service(serve_dir)
                    .with_state(shared_state);

                println!("Listening on port 8000");
                let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
                if let Err(e) = axum::serve(listener, app).await {
                    eprintln!("Server run error: {e}");
                }
            }
        },
    }

    Ok(())
}
