use std::path::PathBuf;

use uuid::Uuid;

pub struct File {
    pub name: String,
    pub upload_key: Uuid,
    pub path: PathBuf,
}

impl File {
    pub fn new(name: String, upload_key: Uuid, path: PathBuf) -> Self {
        Self {
            name,
            upload_key,
            path,
        }
    }
}
