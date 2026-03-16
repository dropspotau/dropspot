use async_trait::async_trait;
use tokio::io::{BufReader, BufWriter};

use crate::{db::File, storage::Storage};

pub struct S3Storage {}

#[async_trait]
impl Storage for S3Storage {
    async fn get_upload_writer(&self, file: &File) -> Result<BufWriter<tokio::fs::File>, ()> {
        let Ok(io_file) = tokio::fs::File::create(file.get_path()).await else {
            return Err(());
        };

        Ok(BufWriter::new(io_file))
    }

    async fn get_download_reader(&self, file: &File) -> Result<BufReader<tokio::fs::File>, ()> {
        let Ok(io_file) = tokio::fs::File::open(file.get_path()).await else {
            return Err(());
        };

        Ok(BufReader::new(io_file))
    }
}
