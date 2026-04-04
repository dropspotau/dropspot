use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct GcsIntegration {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub bucket_name: String,
    pub is_active: bool,
}

pub async fn get_gcs_integration(
    pool: &PgPool,
    organisation_id: &Uuid,
) -> Result<Option<GcsIntegration>, sqlx::Error> {
    sqlx::query_as!(
        GcsIntegration,
        r#"
            select
              id,
              organisation_id,
              bucket_name,
              is_active
            from gcs_integration
            where organisation_id = $1
            limit 1
        "#,
        organisation_id
    )
    .fetch_optional(pool)
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
    sqlx::query_as!(
        GcsIntegration,
        r#"
            insert into gcs_integration (organisation_id, bucket_name)
            values ($1, $2)
            on conflict (organisation_id)
            do update set
                bucket_name = $2
            returning id, organisation_id, bucket_name, is_active
        "#,
        organisation_id,
        bucket_name,
    )
    .fetch_one(pool)
    .await
}
