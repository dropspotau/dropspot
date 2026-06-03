use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{db::File, storage::Storage};

#[allow(dead_code)]
pub struct S3Storage {
    pub bucket_name: String,
}

#[async_trait]
impl Storage for S3Storage {
    async fn get_upload_writer(
        &self,
        _file: &File,
    ) -> Result<Box<dyn AsyncWrite + Unpin + Send>, ()> {
        todo!("S3 storage is not yet implemented")
    }

    async fn finish_upload(&self, _file: &File) -> Result<(), ()> {
        todo!("S3 storage is not yet implemented")
    }

    async fn get_download_reader(
        &self,
        _file: &File,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send>, ()> {
        todo!("S3 storage is not yet implemented")
    }

    async fn delete(&self, _file: &File) -> Result<(), ()> {
        todo!("S3 storage is not yet implemented")
    }
}
