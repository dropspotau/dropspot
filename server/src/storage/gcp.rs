use async_trait::async_trait;
use tokio::io::{BufReader, BufWriter};

use crate::{db::File, storage::Storage};

pub struct GcpStorage {}

#[async_trait]
impl Storage for GcpStorage {
    async fn get_upload_writer(&self, file: &File) -> Result<BufWriter<tokio::fs::File>, ()> {
        todo!("Implement GCP adapter writer")
    }

    async fn get_download_reader(&self, file: &File) -> Result<BufReader<tokio::fs::File>, ()> {
        todo!("Implement GCP adapter writer")
    }
}
