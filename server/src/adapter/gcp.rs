use tokio::io::{BufReader, BufWriter};

use crate::{adapter::Adapter, db::File};

pub struct GcpAdapter {}

impl Adapter for GcpAdapter {
    fn get_upload_writer(
        &self,
        file: &File,
    ) -> impl Future<Output = Result<BufWriter<tokio::fs::File>, ()>> + Send {
        async { todo!("Implement GCP adapter writer") }
    }

    fn get_download_reader(
        &self,
        file: &File,
    ) -> impl Future<Output = Result<BufReader<tokio::fs::File>, ()>> + Send {
        async { todo!("Implement GCP adapter writer") }
    }
}
