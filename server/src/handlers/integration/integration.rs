use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct LocalIntegration {
    pub id: Uuid,
    pub organisation_id: Uuid,
    pub upload_path: String,
    pub is_active: bool,
}

pub async fn get_local_integration(
    pool: &PgPool,
    organisation_id: &Uuid,
) -> Result<Option<LocalIntegration>, sqlx::Error> {
    sqlx::query_as!(
        LocalIntegration,
        r#"
            select
              id,
              organisation_id,
              upload_path,
              is_active
            from local_integration
            where organisation_id = $1
            limit 1
        "#,
        organisation_id
    )
    .fetch_optional(pool)
    .await
}
