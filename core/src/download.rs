use std::io::{BufWriter, Cursor, Write};

use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::constants::ENDPOINT;
use crate::encryption::{DecryptionError, Encryption, decrypt_file};

#[derive(Serialize, Deserialize, Debug)]
pub struct Download {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
}

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
    encryption: &Encryption,
    writer: impl Write,
) -> Result<(), DownloadError> {
    // Request a download URL
    let download = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file/{file_id}/download"))
        .send()
        .await
        .map_err(DownloadError::RequestError)?
        .json::<Download>()
        .await
        .map_err(DownloadError::RequestError)?;

    // Actually download the file
    let download_id = download.id;
    let mut stream = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/download/{download_id}/download"))
        .send()
        .await
        .map_err(DownloadError::RequestError)?
        .bytes_stream()
        .map(move |bytes| {
            if let Err(e) = bytes {
                return Err(e);
            };

            bytes.map(|bytes| bytes.to_vec())
        });

    let mut massive_buffer = Vec::<u8>::new();

    while let Some(bytes) = stream.next().await {
        if let Err(e) = bytes {
            eprintln!("Failed to download file: {e:?}");
            return Err(DownloadError::RequestError(e));
        };

        let bytes = bytes.unwrap();
        massive_buffer.extend_from_slice(bytes.as_ref());
    }

    let reader = Cursor::new(massive_buffer);

    if let Err(e) = decrypt_file(&encryption, reader, writer) {
        eprintln!("Failed to decrypt file: {e:?}");
        return Err(DownloadError::DecryptionError(e));
    };

    Ok(())
}

// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn download_js(file_id: String, encryption: Encryption) -> Result<Vec<u8>, JsError> {
    let file_id = Uuid::parse_str(&file_id)
        .map_err(|err| JsError::new(&format!("Invalid passed to download_js: {err}")))?;
    let mut buffer = Vec::<u8>::new();
    let writer = BufWriter::new(&mut buffer);

    let download = download(file_id, &encryption, writer).await;

    if let Err(e) = download {
        return Err(JsError::new(&format!("{e:?}")));
    };

    Ok(buffer)
}
