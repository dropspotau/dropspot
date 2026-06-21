use std::time::Duration;

use sqlx::PgPool;

use crate::db::{
    File, expire_file, get_default_organisation, get_files_to_expire, get_integration_by_slug,
    get_organisation_for_user,
};
use crate::state::AppState;
use crate::storage::get_storage;

// Deletes a file from disk
async fn delete_file(pool: &PgPool, file: &File) -> Result<(), ()> {
    let organisation = match file.created_by_id {
        Some(id) => get_organisation_for_user(pool, &id).await,
        None => get_default_organisation(pool).await,
    };

    if let Err(e) = organisation {
        tracing::error!("Could not find organisation for file's uploader: {e}");
        return Err(());
    }

    let organisation = Some(organisation.unwrap());
    let Ok(integration) =
        get_integration_by_slug(pool, &organisation.unwrap().id, &file.storage).await
    else {
        tracing::error!("Integration not found for organisation");
        return Err(());
    };

    if let Err(e) = expire_file(pool, &file.id).await {
        tracing::error!("Failed to mark file {} as expired: {e}", file.id);
        return Err(());
    };

    let storage = get_storage(&integration.data);
    storage.delete(file).await
}

// Continually watches for expired files, deleting them from disk if expired
pub async fn watch_for_files(state: AppState) {
    let pool = state.get_pool();

    loop {
        tracing::info!("Watching for files...");
        tokio::time::sleep(Duration::new(1, 0)).await;

        let expired_files = get_files_to_expire(pool).await;

        if let Err(e) = expired_files {
            tracing::error!("Failed to get expired files: {e}");
            continue;
        };

        let expired_files = expired_files.unwrap().into_iter().collect::<Vec<_>>();

        for file in expired_files {
            tracing::info!("Deleting file {}", file.id);

            if delete_file(pool, &file).await.is_err() {
                tracing::error!("Failed to delete file: {}", file.id);
            }
        }
    }
}
