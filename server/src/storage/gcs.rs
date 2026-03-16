use async_trait::async_trait;
use google_cloud_storage::client::{Storage as GoogleCloudStorage, StorageControl};
use tokio::io::{BufReader, BufWriter};

use crate::{db::File, storage::Storage};

pub struct GcsStorage {}

#[async_trait]
impl Storage for GcsStorage {
    async fn get_upload_writer(&self, file: &File) -> Result<BufWriter<tokio::fs::File>, ()> {
        let Ok(client) = GoogleCloudStorage::builder().build().await else {
            return Err(());
        };

        let control = StorageControl::builder().build().await?;

        let bucket_name = "dropspot-upload-filese";
        let path = file.get_path();

        let object = client
            .write_object(bucket_name, file.id, b"")
            .send_buffered()
            .await?;
        println!("object successfully uploaded {object:?}");

        Err(())
    }

    async fn get_download_reader(&self, file: &File) -> Result<BufReader<tokio::fs::File>, ()> {
        todo!("Implement GCP adapter writer")
    }
}
