use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, BufReader, BufWriter};

use crate::{db::File, storage::Storage};

pub struct LocalStorage {}

#[async_trait]
impl Storage for LocalStorage {
    async fn get_upload_writer(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncWrite + Unpin + Send>, ()> {
        let Ok(io_file) = tokio::fs::File::create(file.get_path()).await else {
            return Err(());
        };

        Ok(Box::new(BufWriter::new(io_file)))
    }

    async fn get_download_reader(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send>, ()> {
        let Ok(io_file) = tokio::fs::File::open(file.get_path()).await else {
            return Err(());
        };

        Ok(Box::new(BufReader::new(io_file)))
    }
}
