use sqlx::PgPool;
use uuid::Uuid;

use crate::db::File;
use crate::download::Download;
use crate::upload::Upload;

pub struct State {
    uploads: Vec<Upload>,
    files: Vec<File>,
    downloads: Vec<Download>,
    pool: PgPool,
}

impl State {
    pub fn new(pool: PgPool) -> Self {
        Self {
            uploads: vec![],
            files: vec![],
            downloads: vec![],
            pool,
        }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn add_upload(&mut self, upload: Upload) {
        self.uploads.push(upload);
    }

    pub fn remove_uploads(&mut self, keys: &[Uuid]) {
        self.uploads.retain(|upload| !keys.contains(&upload.id));
    }

    pub fn get_uploads(&self) -> &[Upload] {
        &self.uploads
    }

    pub fn get_upload_by_id(&self, id: &Uuid) -> Option<&Upload> {
        self.uploads.iter().find(|upload| &upload.id == id)
    }

    pub fn get_file_by_id(&self, id: &Uuid) -> Option<&File> {
        self.files.iter().find(|file| &file.id == id)
    }

    pub fn add_download(&mut self, download: Download) {
        self.downloads.push(download);
    }

    pub fn get_downloads(&self) -> &[Download] {
        &self.downloads
    }

    pub fn get_download_by_id(&self, id: &Uuid) -> Option<&Download> {
        self.downloads.iter().find(|download| &download.id == id)
    }
}
