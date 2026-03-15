use dropspot_core::adapter::AdapterType as ApiAdapterType;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use tokio::io::{BufReader, BufWriter};
use async_trait::async_trait;

use crate::{adapter::s3::S3Adapter, db::File};

use super::gcp::GcpAdapter;
use super::local::LocalAdapter;
use super::s3::S3Adapter;

#[derive(Type, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[sqlx(type_name = "adapter", rename_all = "lowercase")]
pub enum AdapterType {
    Local,
    S3,
    GCP,
}

impl From<ApiAdapterType> for AdapterType {
    fn from(adapter_type: ApiAdapterType) -> Self {
        match adapter_type {
            ApiAdapterType::Local => AdapterType::Local,
            ApiAdapterType::S3 => AdapterType::S3,
            ApiAdapterType::GCP => AdapterType::GCP,
        }
    }
}

// Implmented to handle where a file is uploaded to
#[async_trait]
pub trait Adapter: Sync + Send {
    async fn get_upload_writer(
        &self,
        file: &File,
    ) -> Result<BufWriter<tokio::fs::File>>;

    async fn get_download_reader(
        &self,
        file: &File,
    ) -> Result<BufReader<tokio::fs::File>, ()>;

pub fn get_adapter(adapter_type: &AdapterType) -> Box<dyn Adapter> {
    match adapter_type {
        AdapterType::Local => Box::new(LocalAdapter {}.into()),
        AdapterType::S3 => Box::new(S3Adapter {}.into()),
        AdapterType::GCP => Box::new(GcpAdapter {}.into()),
    }
}
