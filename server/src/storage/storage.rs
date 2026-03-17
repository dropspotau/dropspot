use async_trait::async_trait;
use dropspot_core::storage::StorageType as ApiStorageType;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use tokio::io::{AsyncRead, AsyncWrite, BufReader, BufWriter};

use crate::db::File;

use super::gcs::GcsStorage;
use super::local::LocalStorage;
use super::s3::S3Storage;

#[derive(Type, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "storage", rename_all = "lowercase")]
pub enum StorageType {
    Local,
    S3,
    GCS,
}

impl From<ApiStorageType> for StorageType {
    fn from(storage_type: ApiStorageType) -> Self {
        match storage_type {
            ApiStorageType::Local => StorageType::Local,
            ApiStorageType::S3 => StorageType::S3,
            ApiStorageType::GCS => StorageType::GCS,
        }
    }
}

// Implmented to handle where a file is uploaded to
#[async_trait]
pub trait Storage: Sync + Send {
    async fn get_upload_writer(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncWrite + Unpin + Send>, ()>;

    async fn get_download_reader(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send>, ()>;
}

pub fn get_storage(storage_type: &StorageType) -> Box<dyn Storage> {
    match storage_type {
        StorageType::Local => Box::new(LocalStorage {}),
        StorageType::S3 => Box::new(S3Storage {}),
        StorageType::GCS => Box::new(GcsStorage {}),
    }
}
