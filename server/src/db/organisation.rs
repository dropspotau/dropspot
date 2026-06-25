use chrono::{DateTime, Utc};
use dropspot_core::integration::integration::{IntegrationData, LocalIntegrationData};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db::upsert_integration, storage::StorageType};

use super::types::Id;

pub struct Organisation {
    pub id: Uuid,
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub created_at: DateTime<Utc>,
}

pub const DEFAULT_ORGANISATION_NAME: &str = "Default";

pub async fn get_default_organisation(pool: &PgPool) -> Result<Organisation, sqlx::Error> {
    sqlx::query_as!(
        Organisation,
        r#"
            select
              id,
              name,
              created_at
            from dropspot.organisation
            where name = $1
        "#,
        DEFAULT_ORGANISATION_NAME
    )
    .fetch_one(pool)
    .await
}

pub async fn get_organisation_by_id(pool: &PgPool, id: &Uuid) -> Result<Organisation, sqlx::Error> {
    sqlx::query_as!(
        Organisation,
        r#"
            select
              id,
              name,
              created_at
            from dropspot.organisation
            where id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_organisation_for_user(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<Organisation, sqlx::Error> {
    sqlx::query_as!(
        Organisation,
        r#"
            select
              organisation.id,
              organisation.name,
              organisation.created_at
            from dropspot.organisation organisation
            left join dropspot.users users
            on users.organisation_id = organisation.id
            where users.id = $1
            limit 1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}

#[allow(dead_code)]
pub async fn create_organisation(pool: &PgPool, name: &str) -> Result<Organisation, sqlx::Error> {
    let id = sqlx::query_as!(
        Id,
        r#"
            insert into dropspot.organisation (name)
            values ($1)
            returning id
        "#,
        name,
    )
    .fetch_one(pool)
    .await?;

    // Create the local integration as a starting point
    upsert_integration(
        pool,
        &id.id,
        &StorageType::Local,
        true,
        &IntegrationData::Local(LocalIntegrationData {
            folder: "files".to_owned(),
        }),
        None,
    )
    .await?;

    get_organisation_by_id(pool, &id.id).await
}
