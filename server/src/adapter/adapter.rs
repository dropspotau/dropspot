use dropspot_core::adapter::AdapterType as ApiAdapterType;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use tokio::io::{BufReader, BufWriter};

use crate::db::File;

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
pub trait Adapter {
    fn get_upload_writer(
        &self,
        file: &File,
    ) -> impl Future<Output = Result<BufWriter<tokio::fs::File>, ()>> + Send;

    fn get_download_reader(
        &self,
        file: &File,
    ) -> impl Future<Output = Result<BufReader<tokio::fs::File>, ()>> + Send;
}

pub fn get_adapter(adapter_type: &AdapterType) -> impl Adapter + use<> {
    use super::local::LocalAdapter;
    println!("Getting adapter for file: {adapter_type:?}");

    match adapter_type {
        AdapterType::Local => LocalAdapter {},
        AdapterType::S3 => S3Adapter {},
        AdapterType::GCP => GCPAdapter {},
    }
}
