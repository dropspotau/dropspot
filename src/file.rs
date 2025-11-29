use std::path::PathBuf;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct File {
    pub id: Uuid,
    pub name: String,
    pub upload_key: Uuid,
    pub path: PathBuf,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl File {
    pub fn new(name: String, upload_key: Uuid, path: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            upload_key,
            path,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::minutes(1),
        }
    }

    pub fn is_expired(&self) -> bool {
        let is_date_expired = Utc::now() > self.expires_at;
        // TODO(alec): Cound how many download attempts a
        // file has had
        let is_past_download_capacity = false;

        is_date_expired || is_past_download_capacity
    }
}
