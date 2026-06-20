use sqlx::PgPool;
use uuid::Uuid;

use super::types::{Exists, Id};

pub async fn get_onboarding_status(pool: &PgPool, user_id: &Uuid) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_as!(
        Exists,
        r#"
            select exists (
              select id
              from dropspot.onboarding
              where user_id = $1
            ) "exists!"
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(exists.exists)
}

pub async fn record_onboarding_completion(
    pool: &PgPool,
    user_id: &Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query_as!(
        Id,
        r#"
            insert into dropspot.onboarding (user_id, completed_at)
            values ($1, now())
            returning id
        "#,
        user_id,
    )
    .fetch_one(pool)
    .await
    .map(|_id| ())
}
