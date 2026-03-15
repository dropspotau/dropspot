use tokio::io::{BufReader, BufWriter};

use crate::{adapter::Adapter, db::File};

pub struct LocalAdapter {}

impl Adapter for LocalAdapter {
    fn get_upload_writer(
        &self,
        file: &File,
    ) -> impl Future<Output = Result<BufWriter<tokio::fs::File>, ()>> + Send {
        async {
            let Ok(io_file) = tokio::fs::File::create(file.get_path()).await else {
                return Err(());
            };

            Ok(BufWriter::new(io_file))
        }
    }

    fn get_download_reader(
        &self,
        file: &File,
    ) -> impl Future<Output = Result<BufReader<tokio::fs::File>, ()>> + Send {
        async {
            let Ok(io_file) = tokio::fs::File::open(file.get_path()).await else {
                return Err(());
            };

            Ok(BufReader::new(io_file))
        }
    }
}
