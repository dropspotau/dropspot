use std::{thread::sleep, time::Duration};

use crate::db::{get_downloads, get_files, get_uploads};
use crate::state::State;

pub async fn watch_for_files(state: State) {
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
