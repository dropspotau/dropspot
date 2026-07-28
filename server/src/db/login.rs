use sqlx::PgPool;
use uuid::Uuid;

use crate::db::types::Id;

pub async fn record_signin(pool: &PgPool, user_id: &Uuid) -> Result<Id, sqlx::Error> {
    sqlx::query_as!(
        Id,
        r#"
            with login_id as (
                select id
                from dropspot.login
                where user_id = $1
            )
            insert into dropspot.signin (login_id, created_at)
            values ((select id from login_id limit 1), now())
            returning id
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
}
