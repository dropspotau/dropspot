use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct Upload {
    pub id: Uuid,
    pub file_name: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Upload {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

pub async fn get_uploads(pool: &PgPool) -> Result<Vec<Upload>, sqlx::Error> {
    sqlx::query_as!(
        Upload,
        r#"
            select id, file_name, created_at, expires_at
            from upload
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_upload_by_id(pool: &PgPool, id: &Uuid) -> Result<Upload, sqlx::Error> {
    sqlx::query_as!(
        Upload,
        r#"
            select id, file_name, created_at, expires_at
            from upload
            where id = $1
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_upload(pool: &PgPool, name: String) -> Result<Upload, sqlx::Error> {
    let created_at = Utc::now();
    let expires_at = Utc::now() + Duration::minutes(3);

    sqlx::query_as!(
        Upload,
        r#"
            insert into upload (file_name, created_at, expires_at)
            values ($1, $2, $3)
            returning id, file_name, created_at, expires_at
        "#,
        name,
        created_at,
        expires_at
    )
    .fetch_one(pool)
    .await
}
