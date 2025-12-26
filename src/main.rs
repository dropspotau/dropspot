mod db;
mod handlers;
mod state;

use std::{io::Read, thread::sleep, time::Duration};

use clap::Parser;
use state::State;
use uuid::Uuid;

use crate::{
    db::{get_downloads, get_files, get_uploads, run_migrations},
    handlers::{
        handle_file_download, handle_file_request_download, handle_file_request_upload,
        handle_file_upload,
    },
};

async fn watch_for_files(state: State) {
    let pool = state.get_pool();

    loop {
        println!("Watching for files...");
        sleep(Duration::new(1, 0));

        let uploads = get_uploads(pool).await;

        let expired_keys = uploads
            .unwrap()
            .iter()
            .filter(|upload| upload.is_expired())
            .map(|upload| upload.id)
            .collect::<Vec<_>>();

        println!("Removed uploads: {expired_keys:?}");

        let files = get_files(pool).await;

        if let Err(e) = files {
            eprintln!("Failed to get files: {e}");
            continue;
        };

        let downloads = get_downloads(pool).await;

        if let Err(e) = downloads {
            eprintln!("Failed to get downloads: {e}");
            continue;
        };

        println!("This many files: {}", files.unwrap().len());
        println!("This many downloads: {}", downloads.unwrap().len());
    }
}

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
    Download {
        id: String,
    },
    #[command(about = "Watch for files")]
    Watch {},
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let Ok(pool) = db::connect().await else {
        return Err(());
    };

    if let Err(e) = run_migrations(&pool).await {
        eprintln!("Failed to run migrations: {e}");
        return Err(());
    };

    let mut state = State::new(pool);

    match &cli.command {
        Commands::Upload { file } => {
            // Simulate generating an upload URL
            let Ok(upload) = handle_file_request_upload(&mut state).await else {
                return Err(());
            };

            let Ok(mut file) = std::fs::File::open(file) else {
                eprintln!("Failed to open file");
                return Err(());
            };

            let mut buffer = Vec::new();
            if let Err(e) = file.read_to_end(&mut buffer) {
                eprintln!("Failed to read file: {e}");
            }

            // Simulate uploading a file to that URL
            let Ok(file) = handle_file_upload(
                &mut state,
                &upload.id,
                format!("file_{}", upload.id),
                buffer,
            )
            .await
            else {
                eprintln!("Failed to upload file");
                return Err(());
            };

            println!("Uploaded: {}", file.id);
        }
        Commands::Download { id } => {
            let Ok(id) = Uuid::parse_str(id) else {
                eprintln!("Invalid UUID");
                return Err(());
            };

            // Simulate a download
            let download = match handle_file_request_download(&mut state, id).await {
                Ok(download) => download,
                Err(e) => {
                    eprintln!("Failed to request file download: {e}");
                    return Err(());
                }
            };

            let file_stream = match handle_file_download(&mut state, download.id).await {
                Ok(file_stream) => file_stream,
                Err(e) => {
                    eprintln!("Failed to download file: {e}");
                    return Err(());
                }
            };

            for bytes in file_stream {
                println!("{bytes:?}");
            }
        }
        Commands::Watch {} => {
            watch_for_files(state).await;
        }
    }

    Ok(())
}
