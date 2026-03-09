use std::path::PathBuf;

use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::types::Id;
use super::upload::create_upload;

pub struct File {
    pub id: Uuid,
    pub name: String,
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

        is_date_expired || is_past_download_capacity
    }

    pub fn get_path(&self) -> PathBuf {
        PathBuf::from(FILES_DIR).join(self.path.clone())
    }

    pub fn get_formatted_size(&self) -> String {
        let size = self.size as f64;
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        let formatted_size = if size >= GB {
            format!("{:.2} GB", size / GB)
        } else if size >= MB {
            format!("{:.2} MB", size / MB)
        } else if size >= KB {
            format!("{:.2} KB", size / KB)
        } else {
            format!("{size:.0} B")
        };

        formatted_size
    }

    pub fn get_remaining_downloads(&self) -> i32 {
        self.max_downloads - self.download_count
    }

    pub fn get_extension(&self) -> String {
        self.name.split('.').last().unwrap_or("txt").to_string()
    }

    // Deletes the actual file
    pub fn delete_file(&self) -> Result<(), ()> {
        let path = self.get_path();

        // TODO(alec): Handle different providers: filesystem, S3, GCP bucket etc.
        match std::fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(_e) => Err(()),
        }
    }
}

pub async fn get_files(pool: &PgPool) -> Result<Vec<File>, sqlx::Error> {
    sqlx::query_as!(
        File,
        r#"
            select
              file.id,
              file.name,
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
            having file.max_downloads >= count(download.id) and now() < file.expires_at
            order by file.created_at asc
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
            having file.max_downloads < count(download.id) or now() > file.expires_at
            order by file.created_at asc
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
            having file.id = $1 and file.max_downloads >= count(download.id) and now() < file.expires_at
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_file(
    pool: &PgPool,
    name: &str,
    path: &str,
    size: i64,
) -> Result<File, sqlx::Error> {
    let created_at = Utc::now();
    let expires_at = Utc::now() + Duration::minutes(60);
    let max_downloads = 100;

    let id = sqlx::query_as!(
        Id,
        r#"
            insert into file (name, path, size, created_at, expires_at, max_downloads)
            values ($1, $2, $3, $4, $5, $6)
            returning id
        "#,
        name,
        path,
        size,
        created_at,
        expires_at,
        max_downloads
    )
    .fetch_one(pool)
    .await?;

    create_upload(pool, &id.id).await?;
    get_file_by_id(pool, &id.id).await
}

pub async fn delete_files(pool: &PgPool, ids: &[Uuid]) -> Result<Vec<Uuid>, sqlx::Error> {
    let ids = sqlx::query_as!(
        Id,
        r#"
            delete from file
            where id = any($1)
            returning id
        "#,
        ids
    )
    .fetch_all(pool)
    .await?;

    Ok(ids.into_iter().map(|row| row.id).collect())
}
