use std::{
    env::temp_dir,
    io::{Cursor, Read},
};

use async_trait::async_trait;
use bytes::Bytes;
use google_cloud_storage::{
    client::Storage as GoogleCloudStorage, streaming_source::StreamingSource,
};
use tokio::io::{AsyncRead, AsyncWrite, BufWriter};

use crate::{db::File, storage::Storage};

pub struct GcsStorage {}

//
// TODO(alec): At some point, it'd be nice to make the GCS writer not rely on reading from a temp file for GCS uploads.
// It's not a HUGE deal, but would be interesting to work on. We'd want to make get_upload_writer
// return a GcsWriter whose cursor can be written to, but at this point I'm really bikeshedding
// things so I'll postpone it.
//

struct GcsWriter {
    cursor: Cursor<Vec<u8>>,
}

impl StreamingSource for GcsWriter {
    type Error = std::convert::Infallible;

    async fn next(&mut self) -> Option<Result<bytes::Bytes, Self::Error>> {
        let mut buffer = Vec::<u8>::new();
        let bytes_read = match self.cursor.read(&mut buffer) {
            Ok(bytes_read) => bytes_read,
            Err(e) => panic!("Failed to read from GCS writer {e:?}"),
        };

        if bytes_read == 0 {
            return None;
        }

        Some(Ok(Bytes::from_owner(buffer)))
    }
}

#[async_trait]
impl Storage for GcsStorage {
    async fn get_upload_writer(
        &self,
        file: &File,
    ) -> Result<Box<dyn AsyncWrite + Unpin + Send>, ()> {
        let temp_dir = temp_dir();

        let Ok(file) = tokio::fs::File::create(temp_dir.join(file.id.to_string())).await else {
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

        let Ok(temp_file) = tokio::fs::File::open(temp_dir.join(file.id.to_string())).await else {
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

        let bucket_name = "dropspot-upload-files".to_owned();
        let artifact_path = format!("projects/_/buckets/{bucket_name}");
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
