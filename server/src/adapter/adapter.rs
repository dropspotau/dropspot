use tokio::io::{BufReader, BufWriter};

use crate::db::File;

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

pub fn get_adapter(file: &File) -> impl Adapter + use<> {
    use super::local::LocalAdapter;
    println!("Getting adapter for file: {}", file.name);

    LocalAdapter {}
}
