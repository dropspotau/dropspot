use std::net::IpAddr;

use chrono::{DateTime, Duration, Utc};
use sqlx::{PgPool, types::ipnetwork::IpNetwork};
use uuid::Uuid;

use super::types::Id;

/// A download attempt for a file
pub struct Download {
    pub id: Uuid,
    pub file_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub created_by_id: Option<Uuid>,
    pub created_by_name: Option<String>,
    pub download_ip: IpAddr,
    pub expires_at: DateTime<Utc>,
}

impl Download {
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn created_at_slash_date(&self) -> String {
        format!("{}", self.created_at.format("%d/%m/%Y"))
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
                download.download_ip as "download_ip: IpAddr",
                download.expires_at
            from dropspot.download download
            left join dropspot.users users
            on users.id = download.created_by_id
            where download.id = $1
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_downloads_for_file(
    pool: &PgPool,
    file_id: &Uuid,
) -> Result<Vec<Download>, sqlx::Error> {
    // NOTE(alec): There's some funky stuff going on when filtering by download.file_id despite it
    // being non-nullable. Running EXPLAIN on this query results in a RIGHT JOIN being done instead
    //
    // I think this might be a SQLx bug?
    //
    // As a workaround, all the download table rows are enforced as non-nullable because they really
    // are
    sqlx::query_as!(
        Download,
        r#"
            select 
                download.id "id!",
                download.file_id "file_id!",
                download.created_at "created_at!",
                users.id "created_by_id?",
                users.email "created_by_name?",
                download.download_ip "download_ip!: IpAddr",
                download.expires_at "expires_at!"
            from dropspot.download download
            left join dropspot.users users
            on users.id = download.created_by_id
            where download.file_id = $1::uuid
        "#,
        file_id
    )
    .fetch_all(pool)
    .await
}

pub async fn create_download(
    pool: &PgPool,
    file_id: &Uuid,
    created_by_id: Option<Uuid>,
    download_ip: IpAddr,
) -> Result<Download, sqlx::Error> {
    let created_at = Utc::now();
    let expires_at = Utc::now() + Duration::minutes(3);
    let download_ip = IpNetwork::from(download_ip);

    let id = sqlx::query_as!(
        Id,
        r#"
            insert into dropspot.download (file_id, created_at, created_by_id, download_ip, expires_at)
            values ($1, $2, $3, $4, $5)
            returning id
        "#,
        file_id,
        created_at,
        created_by_id,
        download_ip,
        expires_at
    )
    .fetch_one(pool)
    .await?;

    get_download_by_id(pool, &id.id).await
}
