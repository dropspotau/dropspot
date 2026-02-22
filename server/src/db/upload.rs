use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Upload {
    pub id: Uuid,
    pub file_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub upload_started_at: Option<DateTime<Utc>>,
    pub upload_finished_at: Option<DateTime<Utc>>,
    pub has_uploaded: bool,
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
            select id, file_id, created_at, expires_at, upload_started_at, upload_finished_at, has_uploaded
            from upload
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_upload_by_file_id(pool: &PgPool, id: &Uuid) -> Result<Upload, sqlx::Error> {
    sqlx::query_as!(
        Upload,
        r#"
            select id, file_id, created_at, expires_at, upload_started_at, upload_finished_at, has_uploaded
            from upload
            where file_id = $1
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_upload(pool: &PgPool, file_id: &Uuid) -> Result<Upload, sqlx::Error> {
    let created_at = Utc::now();
    let expires_at = Utc::now() + Duration::minutes(3);

    sqlx::query_as!(
        Upload,
        r#"
            insert into upload (file_id , created_at, expires_at)
            values ($1, $2, $3)
            returning id, file_id, created_at, expires_at, upload_started_at, upload_finished_at, has_uploaded
        "#,
        file_id,
        created_at,
        expires_at
    )
    .fetch_one(pool)
    .await
}

pub async fn start_upload(pool: &PgPool, id: &Uuid) -> Result<Upload, sqlx::Error> {
    sqlx::query_as!(
        Upload,
        r#"
            update upload
            set upload_started_at = now()
            where id = $1
            returning id, file_id, created_at, expires_at, upload_started_at, upload_finished_at, has_uploaded
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn finish_upload(pool: &PgPool, id: &Uuid) -> Result<Upload, sqlx::Error> {
    sqlx::query_as!(
        Upload,
        r#"
            update upload
            set upload_finished_at = now()
            where id = $1
            returning id, file_id, created_at, expires_at, upload_started_at, upload_finished_at, has_uploaded
        "#,
        id
    )
    .fetch_one(pool)
    .await
}
