use uuid::Uuid;

use crate::file::File;
use crate::upload::Upload;

pub struct State {
    pub uploads: Vec<Upload>,
    pub files: Vec<File>,
}

impl State {
    pub fn new() -> Self {
        Self {
            uploads: vec![],
            files: vec![],
        }
    }

    pub fn add_upload(&mut self, upload: Upload) {
        self.uploads.push(upload);
    }

    pub fn remove_uploads(&mut self, keys: &[Uuid]) {
        self.uploads.retain(|upload| !keys.contains(&upload.key));
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }
}
