use std::path::Path;

use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, BufReader, BufWriter};

use crate::{db::File, storage::Storage};

pub struct LocalStorage {
    pub folder: String,
}

#[async_trait]
impl Storage for LocalStorage {
    async fn get_upload_writer(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncWrite + Unpin + Send>, ()> {
        let path = Path::new(&self.folder).join(&file.path);

        let Ok(io_file) = tokio::fs::File::create(path).await else {
            return Err(());
        };

        Ok(Box::new(BufWriter::new(io_file)))
    }

    async fn finish_upload(&self, _file: &File) -> Result<(), ()> {
        // Do nothing, the writer writes to the file automatically
        Ok(())
    }

    async fn get_download_reader(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send>, ()> {
        let path = Path::new(&self.folder).join(&file.path);

        let Ok(io_file) = tokio::fs::File::open(path).await else {
            return Err(());
        };

        Ok(Box::new(BufReader::new(io_file)))
    }

    async fn delete(&self, file: &File) -> Result<(), ()> {
        let path = Path::new(&self.folder).join(&file.path);

        tokio::fs::remove_file(path).await.map_err(|_e| ())
    }
}
