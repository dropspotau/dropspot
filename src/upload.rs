use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Upload {
    pub key: Uuid,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Upload {
    pub fn generate() -> Upload {
        let key = Uuid::new_v4();
        let created_at = Utc::now();
        let expires_at = created_at + chrono::Duration::seconds(3);

        Upload {
            key,
            created_at,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
