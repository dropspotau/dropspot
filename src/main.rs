mod file;
mod state;
mod upload;

use std::{thread::sleep, time::Duration};

use state::State;
use upload::Upload;

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
    }
}

fn main() {
    println!("Welcome to DropSpot!");
    let mut state = State::new();

    for _ in 0..3 {
        let upload = Upload::generate();
        state.add_upload(upload);
    }

    watch_for_files(state);
}
