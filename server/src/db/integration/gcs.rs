use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::types::Id;

#[derive(Clone, Serialize, Deserialize)]
pub struct GcsIntegration {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub bucket_name: String,
}

pub async fn get_gcs_integration(
    pool: &PgPool,
    organisation_id: &Uuid,
) -> Result<GcsIntegration, sqlx::Error> {
    sqlx::query_as!(
        GcsIntegration,
        r#"
            select
              id,
              organisation_id,
              bucket_name
            from gcs_integration
            where organisation_id = $1
            limit 1
        "#,
        organisation_id
    )
    .fetch_one(pool)
    .await
}

struct Password {
    password: String,
}

pub async fn upsert_gcs_integration(
    pool: &PgPool,
    organisation_id: &Uuid,
    bucket_name: &str,
) -> Result<GcsIntegration, sqlx::Error> {
    let id = sqlx::query_as!(
        Id,
        r#"
            insert into gcs_integration (organisation_id, bucket_name)
            values ($1, $2)
            on conflict (organisation_id)
            do update set
                bucket_name = $2
            returning id
        "#,
        organisation_id,
        bucket_name,
    )
    .fetch_one(pool)
    .await?;

    get_gcs_integration(pool, &id.id).await
}
