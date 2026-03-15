use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use crate::db::File;

// Implmented to handle where a file is uploaded to
pub trait Adapter {
    fn get_upload_writer(
        &self,
        file: &File,
    ) -> impl Future<Output = Result<BufWriter<tokio::fs::File>, ()>> + Send;
    async fn get_download_reader(&self, file: &File) -> Result<impl AsyncReadExt, ()>;
}

pub fn get_adapter(file: &File) -> impl Adapter + use<> {
    use super::local::LocalAdapter;
    println!("Getting adapter for file: {}", file.name);

    LocalAdapter {}
}
