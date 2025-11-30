use std::path::PathBuf;

use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct File {
    pub id: Uuid,
    pub name: String,
    pub upload_id: Uuid,
    pub path: PathBuf,
    pub size: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

const FILES_DIR: &'static str = "files";

impl File {
    pub fn is_expired(&self) -> bool {
        let is_date_expired = Utc::now() > self.expires_at;
        // TODO(alec): Cound how many download attempts a
        // file has had
        let is_past_download_capacity = false;

        is_date_expired || is_past_download_capacity
    }

    pub fn get_path(&self) -> PathBuf {
        PathBuf::from(FILES_DIR).join(self.path.clone())
    }
}

pub async fn get_files(pool: &PgPool) -> Result<Vec<File>, sqlx::Error> {
    sqlx::query_as!(
        File,
        r#"
            select id, name, upload_id, path, size, created_at, expires_at
            from file
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn create_file(
    pool: &PgPool,
    name: String,
    upload_id: &Uuid,
    path: String,
    size: i64,
) -> Result<File, sqlx::Error> {
    let created_at = Utc::now();
    let expires_at = Utc::now() + Duration::minutes(3);

    sqlx::query_as!(
        File,
        r#"
            insert into file (name, upload_id, path, size, created_at, expires_at)
            values ($1, $2, $3, $4, $5, $6)
            returning id, name, upload_id, path, size, created_at, expires_at
        "#,
        name,
        upload_id,
        path,
        size,
        created_at,
        expires_at
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_file(pool: &PgPool, id: &Uuid) -> Result<File, sqlx::Error> {
    sqlx::query_as!(
        File,
        r#"
            delete from file
            where id = $1
            returning id, name, upload_id, path, size, created_at, expires_at
        "#,
        id
    )
    .fetch_one(pool)
    .await
}
