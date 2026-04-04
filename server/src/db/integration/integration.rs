use serde::{Deserialize, Serialize};
use sqlx::{PgPool, types::JsonValue};
use uuid::Uuid;

use crate::storage::StorageType;

#[derive(Clone, Serialize, Deserialize)]
pub struct Integration {
    pub id: Uuid,
    pub slug: StorageType,
    pub organisation_id: Uuid,
    pub is_active: bool,
    pub data: JsonValue,
}

pub async fn get_integration_by_slug(
    pool: &PgPool,
    organisation_id: &Uuid,
    slug: &StorageType,
) -> Result<Integration, sqlx::Error> {
    let integrations = sqlx::query_as!(
        Integration,
        r#"
            select
              id,
              slug as "slug: StorageType",
              organisation_id,
              is_active,
              data
            from integration
            where organisation_id = $1
            limit 1
        "#,
        organisation_id
    )
    .fetch_all(pool)
    .await?;

    let integration = integrations.into_iter().find(|i| &i.slug == slug).unwrap();
    Ok(integration)
}

struct Password {
    password: String,
}

pub async fn set_integration_status(
    pool: &PgPool,
    organisation_id: &Uuid,
    slug: &StorageType,
    is_active: bool,
) -> Result<Integration, sqlx::Error> {
    sqlx::query_as!(
        Integration,
        r#"
            update integration
            set is_active = $3
            where organisation_id = $1 and slug = $2::storage
            returning id, slug as "slug: StorageType", organisation_id, is_active, data
        "#,
        organisation_id,
        slug,
        is_active,
    )
    .fetch_one(pool)
    .await
}

pub async fn upsert_integration(
    pool: &PgPool,
    organisation_id: &Uuid,
    slug: &StorageType,
    data: JsonValue,
) -> Result<Integration, sqlx::Error> {
    sqlx::query_as!(
        Integration,
        r#"
            insert into integration (organisation_id, slug, is_active, data)
            values ($1, $2, $3, $4)
            on conflict (organisation_id)
            do update set
                data = $4
            returning id, slug as "slug: StorageType", organisation_id, is_active, data
        "#,
        organisation_id,
        data,
    )
    .fetch_one(pool)
    .await
}
