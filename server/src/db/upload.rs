use std::net::IpAddr;

use chrono::{DateTime, Duration, Utc};
use dropspot_core::{
    integration::integration::Integration as ApiIntegration, upload::PreviewUploadResult,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use sqlx::types::ipnetwork::IpNetwork;
use uuid::Uuid;

use crate::db::{get_integrations, organisation::get_default_organisation};

#[derive(Serialize, Deserialize)]
pub struct Upload {
    pub id: Uuid,
    pub file_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub upload_started_at: Option<DateTime<Utc>>,
    pub upload_finished_at: Option<DateTime<Utc>>,
    pub upload_ip: IpAddr,
    pub has_uploaded: bool,
}

pub async fn get_upload_by_file_id(pool: &PgPool, id: &Uuid) -> Result<Upload, sqlx::Error> {
    sqlx::query_as!(
        Upload,
        r#"
            select
              id,
              file_id,
              created_at,
              expires_at,
              upload_started_at,
              upload_finished_at,
              upload_ip as "upload_ip: IpAddr",
              has_uploaded
            from upload
            where file_id = $1
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_upload(
    pool: &PgPool,
    file_id: &Uuid,
    upload_ip: IpAddr,
) -> Result<Upload, sqlx::Error> {
    let created_at = Utc::now();
    let expires_at = Utc::now() + Duration::minutes(3); // Three minute leeway to upload
    let upload_ip = IpNetwork::from(upload_ip);

    sqlx::query_as!(
        Upload,
        r#"
            insert into upload (file_id, created_at, expires_at, upload_ip)
            values ($1, $2, $3, $4::inet)
            returning 
              id,
              file_id,
              created_at,
              expires_at,
              upload_started_at,
              upload_finished_at,
              upload_ip as "upload_ip: IpAddr",
              has_uploaded
        "#,
        file_id,
        created_at,
        expires_at,
        upload_ip
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
            returning
              id,
              file_id,
              created_at,
              expires_at,
              upload_started_at,
              upload_finished_at,
              upload_ip as "upload_ip: IpAddr",
              has_uploaded
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
            returning
              id,
              file_id,
              created_at,
              expires_at,
              upload_started_at,
              upload_finished_at,
              upload_ip as "upload_ip: IpAddr",
              has_uploaded
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn preview_upload(
    pool: &PgPool,
    organisation_id: Option<&Uuid>,
) -> Result<PreviewUploadResult, sqlx::Error> {
    let organisation_id = match organisation_id {
        Some(id) => id,
        None => &get_default_organisation(pool).await?.id,
    };
    let integrations = get_integrations(pool, organisation_id)
        .await?
        .into_iter()
        .filter(|integration| integration.is_active)
        .map(ApiIntegration::from)
        .collect::<Vec<ApiIntegration>>();

    // TODO(alec): Returning API-specific types from a database call seems a bit strange
    Ok(PreviewUploadResult {
        can_upload: !integrations.is_empty(),
        is_at_file_limit: false,
        file_too_large: false,
        integrations,
    })
}
