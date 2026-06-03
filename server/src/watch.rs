use std::{thread::sleep, time::Duration};

use sqlx::PgPool;

use crate::db::{
    File, get_default_organisation, get_expired_files, get_integration_by_slug,
    get_organisation_for_user,
};
use crate::state::AppState;
use crate::storage::get_storage;

async fn delete_file(pool: &PgPool, file: &File) -> Result<(), ()> {
    let organisation = match file.created_by_id {
        Some(id) => get_organisation_for_user(pool, &id).await,
        None => get_default_organisation(pool).await,
    };

    let organisation = Some(organisation.unwrap());
    let Ok(integration) =
        get_integration_by_slug(pool, &organisation.unwrap().id, &file.storage).await
    else {
        eprintln!("Integration not found for organisation");
        return Err(());
    };

    let storage = get_storage(&integration.data);
    storage.delete(file).await
}

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

        let expired_files = expired_files.unwrap().into_iter().collect::<Vec<_>>();

        for file in expired_files {
            if delete_file(pool, &file).await.is_err() {
                eprintln!("Failed to delete file: {}", file.id);
            }
        }
    }
}
