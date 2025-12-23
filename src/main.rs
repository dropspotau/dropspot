mod db;
mod handlers;
mod state;

use std::{thread::sleep, time::Duration};

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

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Welcome to DropSpot!");
    let Ok(pool) = db::connect().await else {
        return Err(());
    };

    if let Err(e) = run_migrations(&pool).await {
        eprintln!("Failed to run migrations: {e}");
        return Err(());
    };

    let mut state = State::new(pool);
    let mut first_file_id: Option<Uuid> = None;

    for i in 0..3 {
        // Simulate generating an upload URL
        let Ok(upload) = handle_file_request_upload(&mut state).await else {
            continue;
        };

        // Simulate uploading a file to that URL
        let Ok(file) = handle_file_upload(
            &mut state,
            &upload.id,
            format!("file_{i}"),
            format!("This is file {i}").into(),
        )
        .await
        else {
            eprintln!("EEEEE");
            continue;
        };

        first_file_id = Some(file.id.clone());
    }

    // Simulate a download
    let download = match handle_file_request_download(&mut state, first_file_id.unwrap()).await {
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

    watch_for_files(state).await;
    Ok(())
}
