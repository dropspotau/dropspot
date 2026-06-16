use sqlx::PgPool;
use uuid::Uuid;

use super::types::Id;

pub struct Settings {
    pub default_file_expiry_minutes: i32,
    pub default_download_limit: i32,
    pub allow_unauthorised_uploads: bool,
}

pub async fn get_organisation_settings(
    pool: &PgPool,
    organisation_id: &Uuid,
) -> Result<Settings, sqlx::Error> {
    sqlx::query_as!(
        Settings,
        r#"
            select
              default_file_expiry_minutes,
              default_download_limit,
              allow_unauthorised_uploads
            from settings
            where organisation_id = $1
        "#,
        organisation_id
    )
    .fetch_one(pool)
    .await
}

#[allow(dead_code)]
pub async fn create_organisation_settings(
    pool: &PgPool,
    organisation_id: &Uuid,
    default_file_expiry_minutes: i32,
    default_download_limit: i32,
    allow_unauthorised_uploads: bool,
) -> Result<Settings, sqlx::Error> {
    let id = sqlx::query_as!(
        Id,
        r#"
            insert into settings (organisation_id, default_file_expiry_minutes, default_download_limit, allow_unauthorised_uploads)
            values ($1, $2, $3, $4)
            returning organisation_id id
        "#,
        organisation_id,
        default_file_expiry_minutes,
        default_download_limit,
        allow_unauthorised_uploads
    )
    .fetch_one(pool)
    .await?;

    get_organisation_settings(pool, &id.id).await
}

pub async fn update_organisation_settings(
    pool: &PgPool,
    organisation_id: &Uuid,
    default_file_expiry_minutes: i32,
    default_download_limit: i32,
) -> Result<Settings, sqlx::Error> {
    let organisation_id = sqlx::query_as!(
        Id,
        r#"
            update settings
            set default_file_expiry_minutes = $2, default_download_limit = $3
            where organisation_id = $1
            returning organisation_id id
        "#,
        organisation_id,
        default_file_expiry_minutes,
        default_download_limit
    )
    .fetch_one(pool)
    .await?;

    get_organisation_settings(pool, &organisation_id.id).await
}
