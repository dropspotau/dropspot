use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::types::Id;

pub struct Organisation {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

const DEFAULT_ORGANISATION_NAME: &str = "Default";

// TODO(alec): Whenever we handle multiple organisations, we'll need to retrieve them by ID
pub async fn get_organisation(pool: &PgPool) -> Result<Organisation, sqlx::Error> {
    sqlx::query_as!(
        Organisation,
        r#"
            select
              organisation.id,
              organisation.name,
              organisation.created_at
            from organisation
            where name = $1
            limit 1
        "#,
        DEFAULT_ORGANISATION_NAME
    )
    .fetch_one(pool)
    .await
}

pub async fn create_organisation(pool: &PgPool, name: &str) -> Result<Organisation, sqlx::Error> {
    let _id = sqlx::query_as!(
        Id,
        r#"
            insert into organisation (name)
            values ($1)
            returning id
        "#,
        name,
    )
    .fetch_one(pool)
    .await?;

    get_organisation(pool).await
}
