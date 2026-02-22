use std::{thread::sleep, time::Duration};

use crate::db::get_expired_files;
use crate::state::AppState;

pub async fn watch_for_files(state: AppState) {
    let pool = state.get_pool();

    loop {
        println!("Watching for files...");
        sleep(Duration::new(1, 0));

        let expired_files = get_expired_files(pool).await;
        if let Err(e) = expired_files {
            eprintln!("Failed to get expired files: {e}");
            continue;
        };

        let expired_files = expired_files
            .unwrap()
            .into_iter()
            .map(|file| file.id)
            .collect::<Vec<_>>();
        println!("{expired_files:?}");
    }
}
