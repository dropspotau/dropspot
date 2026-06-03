use std::{env::temp_dir, io::Cursor};

use async_trait::async_trait;
use google_cloud_storage::client::{Storage as GoogleCloudStorage, StorageControl};
use tokio::io::{AsyncRead, AsyncWrite, BufWriter};

use crate::{db::File, storage::Storage};

#[allow(dead_code)]
pub struct GcsStorage {
    pub bucket_name: String,
}

//
// TODO(alec): At some point, it'd be nice to make the GCS writer not rely on reading from a temp file for GCS uploads.
// It's not a HUGE deal, but would be interesting to work on. We'd want to make get_upload_writer
// return a GcsWriter whose cursor can be written to, but at this point I'm really bikeshedding
// things so I'll postpone it.
//

#[async_trait]
impl Storage for GcsStorage {
    async fn get_upload_writer(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncWrite + Unpin + Send>, ()> {
        let temp_dir = temp_dir();

        let Ok(file) = tokio::fs::File::create(temp_dir.join(&file.path)).await else {
            eprintln!("Failed to create temporary GCS upload file");
            return Err(());
        };

        Ok(Box::new(BufWriter::new(file)))
    }

    async fn finish_upload(&self, file: &File) -> Result<(), ()> {
        let Ok(client) = GoogleCloudStorage::builder().build().await else {
            return Err(());
        };

        let bucket_name = "dropspot-upload-files".to_owned();
        let artifact_path = format!("projects/_/buckets/{bucket_name}");

        // TODO(alec): Can temp_dir give different results on different calls?
        let temp_dir = temp_dir();

        let Ok(temp_file) = tokio::fs::File::open(temp_dir.join(&file.path)).await else {
            eprintln!("Failed to create temporary GCS upload file");
            return Err(());
        };

        let object = client
            .write_object(&artifact_path, file.id, temp_file)
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

        let artifact_path = format!("projects/_/buckets/{}", self.bucket_name);
        let reader = client.read_object(&artifact_path, &file.path).send().await;

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

    async fn delete(&self, file: &File) -> Result<(), ()> {
        let Ok(client) = StorageControl::builder().build().await else {
            return Err(());
        };
        let artifact_path = format!("projects/_/buckets/{}", self.bucket_name);

        client
            .delete_object()
            .set_bucket(artifact_path)
            .set_object(&file.path)
            .send()
            .await
            .map_err(|_e| ())
    }
}
