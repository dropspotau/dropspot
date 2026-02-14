mod core;
mod server;
mod watch;

use std::io::Read;
use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use clap::Parser;
use futures_util::StreamExt;
use uuid::Uuid;
use watch::watch_for_files;

use crate::core::download::download;
use crate::server::db;
use crate::server::handlers::handle_file_request_upload;
use crate::server::state::AppState;
use crate::{
    core::{upload::upload, validation::validate_file},
    server::handlers::{handle_file_download, handle_file_request_download},
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

    let Ok(pool) = db::connect().await else {
        return Err(());
    };

    let mut state = AppState::new(pool);

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

            if let Err(e) = upload(file_name.clone(), buffer).await {
                eprintln!("Failed to upload file: {e}");
                return Err(());
            }
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

            let download_stream = download_stream.unwrap();


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
                .with_state(shared_state);
        }
    }

    Ok(())
}
