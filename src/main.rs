mod core;
mod server;
mod watch;

use std::io::Read;
use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use clap::Parser;
use futures_util::StreamExt;
use tokio::net::TcpListener;
use uuid::Uuid;

use crate::{
    core::{download::download, upload::upload, validation::validate_file},
    server::{
        db::connect,
        handlers::{
            handle_file_download, handle_file_request_download, handle_file_request_upload,
            handle_file_upload,
        },
        state::AppState,
    },
    watch::watch_for_files,
};

#[derive(Parser)]
#[command(name = "dropspot")]
#[command(about = "A simple file sharing CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    #[command(about = "Upload a file")]
    Upload { file: String },
    #[command(about = "Download a file")]
    Download { id: String },
    #[command(about = "Watch for files")]
    Watch,
    #[command(about = "Run the server")]
    Server,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let Ok(pool) = connect().await else {
        return Err(());
    };

    let state = AppState::new(pool);

    match &cli.command {
        Commands::Upload { file: file_name } => {
            // Simulate generating an upload URL
            let validation = validate_file(file_name);

            if let Err(e) = validation {
                eprintln!("Failed to validate file: {e}");
                return Err(());
            }

            let mut file = validation.unwrap();
            let mut buffer = Vec::new();
            if let Err(e) = file.read_to_end(&mut buffer) {
                eprintln!("Failed to read file: {e}");
                return Err(());
            }

            let upload = upload(file_name.clone(), buffer).await;

            if let Err(e) = upload {
                eprintln!("Failed to upload file: {e}");
                return Err(());
            }

            let upload = upload.unwrap();
            println!("Uploaded file {}", upload.id);
        }
        Commands::Download { id } => {
            let Ok(id) = Uuid::parse_str(id) else {
                eprintln!("Invalid UUID");
                return Err(());
            };

            let download_stream = download(id).await;

            if let Err(e) = download_stream {
                eprintln!("Failed to download file: {e}");
                return Err(());
            }

            let mut download_stream = download_stream.unwrap();

            while let Some(bytes) = download_stream.next().await {
                println!("{bytes:?}");
            }
        }
        Commands::Watch {} => {
            watch_for_files(state).await;
        }
        Commands::Server => {
            let shared_state = Arc::new(state);

            let app = Router::new()
                .route("/upload", post(handle_file_request_upload))
                .route("/upload/{upload_id}", post(handle_file_upload))
                .route("/download{file_id}", get(handle_file_request_download))
                .route("/download/{download_id}", get(handle_file_download))
                .with_state(shared_state);

            println!("Listening on port 8000");
            let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("Server run error: {e}");
            }
        }
    }

    Ok(())
}
