use uuid::Uuid;

use crate::upload::Upload;

pub struct State {
    pub uploads: Vec<Upload>,
}

impl State {
    pub fn new() -> Self {
        Self { uploads: vec![] }
    }

    pub fn add_upload(&mut self, upload: Upload) {
        self.uploads.push(upload);
    }

    pub fn remove_uploads(&mut self, keys: &[Uuid]) {
        self.uploads.retain(|upload| !keys.contains(&upload.key));
    }
}
