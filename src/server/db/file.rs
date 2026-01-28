use std::path::PathBuf;

use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Used in queries that just return an ID
struct FileId {
    id: Uuid,
}

pub struct File {
    pub id: Uuid,
    pub name: String,
    pub upload_id: Uuid,
    pub path: PathBuf,
    pub size: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub max_downloads: i32,
    pub download_count: i32,
}

const FILES_DIR: &'static str = "files";

impl File {
    pub fn is_expired(&self) -> bool {
        let is_date_expired = Utc::now() > self.expires_at;
        let is_past_download_capacity = self.max_downloads <= self.download_count;

        println!(" {is_date_expired} {}", self.expires_at);
        println!("{} {}", self.max_downloads, self.download_count);

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
            select
              file.id,
              file.name,
              file.upload_id,
              file.path,
              file.size,
              file.created_at,
              file.expires_at,
              file.max_downloads,
              count(download.id)::int "download_count!"
            from file
            left join download
            on download.file_id = file.id
            group by file.id
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_expired_files(pool: &PgPool) -> Result<Vec<File>, sqlx::Error> {
    sqlx::query_as!(
        File,
        r#"
            select
              file.id,
              file.name,
              file.upload_id,
              file.path,
              file.size,
              file.created_at,
              file.expires_at,
              file.max_downloads,
              count(download.id)::int "download_count!"
            from file
            left join download
            on download.file_id = file.id
            group by file.id
            having file.max_downloads < count(download.id)
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_file_by_id(pool: &PgPool, id: &Uuid) -> Result<File, sqlx::Error> {
    sqlx::query_as!(
        File,
        r#"
            select
              file.id,
              file.name,
              file.upload_id,
              file.path,
              file.size,
              file.created_at,
              file.expires_at,
              file.max_downloads,
              count(download.id)::int "download_count!"
            from file
            left join download
            on download.file_id = file.id
            group by file.id
            having file.id = $1
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
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
    let max_downloads = 1;

    let id = sqlx::query_as!(
        FileId,
        r#"
            insert into file (name, upload_id, path, size, created_at, expires_at, max_downloads)
            values ($1, $2, $3, $4, $5, $6, $7)
            returning id
        "#,
        name,
        upload_id,
        path,
        size,
        created_at,
        expires_at,
        max_downloads
    )
    .fetch_one(pool)
    .await?;

    get_file_by_id(pool, &id.id).await
}

pub async fn delete_files(pool: &PgPool, ids: &[Uuid]) -> Result<Vec<Uuid>, sqlx::Error> {
    let ids = sqlx::query!(
        r#"
            delete from file
            where id = any($1)
            returning id
        "#,
        ids
    )
    .fetch_all(pool)
    .await?;

    Ok(ids.iter().map(|row| row.id).collect())
}
