use std::path::PathBuf;

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct File {
    pub id: Uuid,
    pub name: String,
    pub upload_id: Uuid,
    pub path: PathBuf,
    pub size: usize,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

const FILES_DIR: &'static str = "files";

impl File {
    pub fn new(name: String, upload_id: Uuid, path: PathBuf, size: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            upload_id,
            path,
            size,
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

    pub fn get_path(&self) -> PathBuf {
        PathBuf::from(FILES_DIR).join(self.path.clone())
    }
}
