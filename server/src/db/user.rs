use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub created_at: DateTime<Utc>,
    pub file_count: i32,
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            select
              users.id,
              users.first_name,
              users.last_name,
              users.created_at,
              count(file.id)::int "file_count!"
            from users
            left join file
            on file.uploaded_by_id = users.id
            group by users.id
            order by users.created_at asc
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, id: &Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            select
              users.id,
              users.first_name,
              users.last_name,
              users.created_at,
              count(file.id)::int "file_count!"
            from users
            left join file
            on file.uploaded_by_id = users.id
            group by users.id
            having users.id = $1
            order by users.created_at asc
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}
