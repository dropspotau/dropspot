mod download;
mod file;
mod handlers;
mod state;
mod upload;

use std::{thread::sleep, time::Duration};

use state::State;
use upload::Upload;
use uuid::Uuid;

use crate::handlers::{handle_file_download, handle_file_request_download, handle_file_upload};

fn watch_for_files(mut state: State) {
    loop {
        println!("Watching for files...");
        sleep(Duration::new(1, 0));
        println!("This many uploads: {}", state.get_uploads().len());

        let expired_keys = state
            .get_uploads()
            .iter()
            .filter(|upload| upload.is_expired())
            .map(|upload| upload.id)
            .collect::<Vec<_>>();

        state.remove_uploads(&expired_keys);

        println!("Removed uploads: {expired_keys:?}");

        println!("This many files: {}", state.get_files().len());
        println!("This many downloads: {}", state.get_downloads().len());
    }
}

fn main() -> Result<(), ()> {
    println!("Welcome to DropSpot!");
    let mut state = State::new();

    let mut first_file_id: Option<Uuid> = None;

    for i in 0..3 {
        // Simulate generating an upload URL
        let upload = Upload::generate();
        let upload_id = upload.id.clone();
        state.add_upload(upload);

        // Simulate uploading a file to that URL
        let Ok(file) = handle_file_upload(&mut state, upload_id, format!("File {i}")) else {
            continue;
        };

        first_file_id = Some(file.id.clone());
        state.add_file(file);
    }

    // Simulate a download
    let Ok(download) = handle_file_request_download(&mut state, first_file_id.unwrap()) else {
        eprintln!("Failed to download file");
        return Err(());
    };

    let Ok(file_stream) = handle_file_download(&mut state, download) else {
        eprintln!("Failed to download file");
        return Err(());
    };

    println!("File stream: {file_stream:?}");

    watch_for_files(state);
    Ok(())
}
