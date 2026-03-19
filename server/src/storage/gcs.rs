use std::{io::Cursor, sync::Arc};

use async_trait::async_trait;
use google_cloud_storage::client::Storage as GoogleCloudStorage;
use tokio::io::{AsyncRead, AsyncWrite, BufWriter};

use crate::{db::File, storage::Storage};

pub struct GcsStorage {}

#[async_trait]
impl Storage for GcsStorage {
    async fn get_upload_writer(
        &self,
        _file: &File,
    ) -> Result<Box<dyn AsyncWrite + Unpin + Send>, ()> {
        let Ok(io_file) = tokio::fs::File::create("upload").await else {
            eprintln!("Could not create GCS writer file");
            return Err(());
        };

        Ok(Box::new(BufWriter::new(io_file)))
    }

    async fn finish_upload(&self, file: &File) -> Result<(), ()> {
        let Ok(client) = GoogleCloudStorage::builder().build().await else {
            return Err(());
        };

        let bucket_name = "dropspot-upload-files".to_owned();
        let artifact_path = format!("projects/_/buckets/{bucket_name}");

        println!("artifact_path: {artifact_path}");

        let Ok(io_file) = tokio::fs::File::open("upload").await else {
            return Err(());
        };

        let object = client
            .write_object(
                &artifact_path,
                file.id,
                // bytes::Bytes::copy_from_slice(&buffer),
                io_file,
            )
            .send_buffered()
            .await;

        if let Err(e) = object {
            eprintln!("Error writing to GCS bucket artifact: {e}");
            return Err(());
        }

        println!("object successfully uploaded {object:?}");

        Ok(())
    }

    async fn get_download_reader(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncRead + Unpin + Send>, ()> {
        let Ok(client) = GoogleCloudStorage::builder().build().await else {
            return Err(());
        };

        let bucket_name = "dropspot-upload-files".to_owned();
        let artifact_path = format!("projects/_/buckets/{bucket_name}");

        println!("artifact_path: {artifact_path}");

        let reader = client.read_object(&artifact_path, file.id).send().await;

        if let Err(e) = reader {
            eprintln!("Error reading from GCS bucket artifact: {e}");
            return Err(());
        }

        // TODO(alec): Figure out a better way than reading the entre file into memory
        // This exists because ReadObjectResponse does not implement AsyncRead
        let mut reader = reader.unwrap();
        let mut buffer = Vec::<u8>::with_capacity(file.size as usize);

        while let Some(chunk) = reader.next().await {
            if let Err(e) = chunk {
                eprintln!("Error reading from GCS bucket artifact: {e}");
                return Err(());
            }

            buffer.extend_from_slice(&chunk.unwrap());
        }

        Ok(Box::new(Cursor::new(buffer)))
    }
}
