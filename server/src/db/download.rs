use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::types::Id;

/// A download attempt for a file
pub struct Download {
    pub id: Uuid,
    pub file_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by_id: Option<Uuid>,
    pub created_by_name: Option<String>,
    pub expires_at: DateTime<Utc>,
}

impl Download {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

pub async fn get_download_by_id(pool: &PgPool, id: &Uuid) -> Result<Download, sqlx::Error> {
    sqlx::query_as!(
        Download,
        r#"
            select 
                download.id,
                download.file_id,
                download.created_at,
                users.id "created_by_id?",
                users.email "created_by_name?",
                download.expires_at
            from download
            left join users
            on users.id = download.created_by_id
            where download.id = $1
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_download(
    pool: &PgPool,
    file_id: &Uuid,
    created_by_id: Option<Uuid>,
) -> Result<Download, sqlx::Error> {
    let created_at = Utc::now();
    let expires_at = Utc::now() + Duration::minutes(3);

    let id = sqlx::query_as!(
        Id,
        r#"
            insert into download (file_id, created_at, created_by_id, expires_at)
            values ($1, $2, $3, $4)
            returning id
        "#,
        file_id,
        created_at,
        created_by_id,
        expires_at
    )
    .fetch_one(pool)
    .await?;

    get_download_by_id(pool, &id.id).await
}
