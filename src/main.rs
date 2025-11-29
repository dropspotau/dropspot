mod file;
mod handlers;
mod state;
mod upload;

use std::{thread::sleep, time::Duration};

use state::State;
use upload::Upload;
use uuid::Uuid;

use crate::{file::File, handlers::handle_file};

fn watch_for_files(mut state: State) {
    loop {
        println!("Watching for files...");
        sleep(Duration::new(1, 0));
        println!("This many uploads: {}", state.uploads.len());

        let expired_keys = state
            .uploads
            .iter()
            .filter(|upload| upload.is_expired())
            .map(|upload| upload.key)
            .collect::<Vec<_>>();

        state.remove_uploads(&expired_keys);

        println!("Removed uploads: {expired_keys:?}");
        println!("This many files: {}", state.files.len());
    }
}

fn main() {
    println!("Welcome to DropSpot!");
    let mut state = State::new();

    for i in 0..3 {
        // Simulate generating an upload URL
        let upload = Upload::generate();
        let upload_key = upload.key.clone();
        state.add_upload(upload);

        // Simulate uploading a file to that URL
        let Ok(file) = handle_file(&mut state, upload_key, format!("File {i}")) else {
            continue;
        };
        state.add_file(file);
    }

    watch_for_files(state);
}
