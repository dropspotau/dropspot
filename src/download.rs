use chrono::{DateTime, Utc};
use uuid::Uuid;

/// A download attempt for a file
pub struct Download {
    pub id: Uuid,
    pub file_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Download {
    pub fn generate(file_id: Uuid) -> Self {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let expires_at = created_at + chrono::Duration::seconds(3);

        Self {
            id,
            file_id,
            created_at,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
