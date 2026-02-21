use std::io::Write;

use bytes::Bytes;
use futures_util::{Stream, StreamExt, TryStreamExt};
use reqwest::Error;
use uuid::Uuid;

use crate::{
    core::encryption::{DecryptionError, Encryption, decrypt_chunk},
    server::handlers::ApiDownload,
};

use super::constants::ENDPOINT;

#[derive(Debug, thiserror::Error)]
pub enum DownloadError {
    #[error("Encryption error: {0}")]
    DecryptionError(DecryptionError),

    #[error("Upload error: {0}")]
    RequestError(reqwest::Error),

    #[error("Server failure: {0}")]
    ServerFailure(reqwest::StatusCode),
}

pub async fn download(
    file_id: Uuid,
    encryption: Encryption,
) -> Result<impl Stream<Item = Result<Vec<u8>, Error>> + use<>, DownloadError> {
    // Request a download URL
    let download = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file/{file_id}/download"))
        .send()
        .await
        .map_err(DownloadError::RequestError)?
        .json::<ApiDownload>()
        .await
        .map_err(DownloadError::RequestError)?;

    // Actually download the file
    let download_id = download.id;
    let stream = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/download/{download_id}/download"))
        .send()
        .await
        .map_err(DownloadError::RequestError)?
        .bytes_stream()
        .map(move |bytes| {
            if let Err(e) = bytes {
                return Err(e);
            };

            let decrypted_bytes = decrypt_chunk(&encryption, &bytes.unwrap().to_vec());
            if let Err(e) = decrypted_bytes {
                panic!("Failed to decrypt file: {e:?}");
            }

            decrypted_bytes.map_err(|_e| panic!(""))
        });

    Ok(stream)
}
