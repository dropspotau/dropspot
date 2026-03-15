use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::db::File;

// Implmented to handle where a file is uploaded to
pub trait Adapter {
    async fn get_upload_writer(&self, file: &File) -> Result<impl AsyncWriteExt, ()>;
    async fn get_download_reader(&self, file: &File) -> Result<impl AsyncReadExt, ()>;
}

pub fn get_adapter(file: &File) -> impl Adapter + use<> {
    use super::local::LocalAdapter;
    println!("Getting adapter for file: {}", file.name);

    LocalAdapter {}
}
