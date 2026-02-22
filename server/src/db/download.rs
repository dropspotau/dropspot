use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// A download attempt for a file
pub struct Download {
    pub id: Uuid,
    pub file_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Download {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

pub async fn get_downloads(pool: &PgPool) -> Result<Vec<Download>, sqlx::Error> {
    sqlx::query_as!(
        Download,
        r#"
            select id, file_id, created_at, expires_at
            from download
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_download_by_id(pool: &PgPool, id: &Uuid) -> Result<Download, sqlx::Error> {
    sqlx::query_as!(
        Download,
        r#"
            select id, file_id, created_at, expires_at
            from download
            where id = $1
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_download(pool: &PgPool, file_id: &Uuid) -> Result<Download, sqlx::Error> {
    let created_at = Utc::now();
    let expires_at = Utc::now() + Duration::minutes(3);

    sqlx::query_as!(
        Download,
        r#"
            insert into download (file_id, created_at, expires_at)
            values ($1, $2, $3)
            returning id, file_id, created_at, expires_at
        "#,
        file_id,
        created_at,
        expires_at
    )
    .fetch_one(pool)
    .await
}
