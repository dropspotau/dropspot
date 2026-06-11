use std::io::{BufWriter, Cursor, Write};

use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;
use crate::encryption::{Encryption, decrypt_file};
use crate::error::{ApiError, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Download {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
}

pub async fn download(
    file_id: &Uuid,
    encryption: &Encryption,
    writer: impl Write,
    auth: Option<&Authentication>,
) -> Result<(), Error> {
    // Request a download URL
    let headers = get_auth_headers(auth);
    let download = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file/{file_id}/download"))
        .headers(headers)
        .send()
        .await
        .map_err(|_e| Error::NetworkError)?;

    if !download.status().is_success() {
        return Err(download
            .json::<ApiError>()
            .await
            .map(Error::ApiError)
            .map_err(|_e| Error::JsonError)?);
    }

    let download = download
        .json::<Download>()
        .await
        .map_err(|_e| Error::JsonError)?;

    // Actually download the file
    let headers = get_auth_headers(auth);
    let download_id = download.id;
    let mut stream = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/download/{download_id}/download"))
        .headers(headers)
        .send()
        .await
        .map_err(|_e| Error::NetworkError)?
        .bytes_stream()
        .map(move |bytes| {
            if let Err(e) = bytes {
                return Err(e);
            };

            bytes.map(|bytes| bytes.to_vec())
        });

    let mut massive_buffer = Vec::<u8>::new();

    while let Some(bytes) = stream.next().await {
        if let Err(_e) = bytes {
            return Err(Error::NetworkError);
        };

        let bytes = bytes.unwrap();
        massive_buffer.extend_from_slice(bytes.as_ref());
    }

    let reader = Cursor::new(massive_buffer);

    if let Err(_e) = decrypt_file(&encryption, reader, writer) {
        return Err(Error::EncryptionError);
    };

    Ok(())
}

// #[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = download)]
pub async fn download_js(
    file_id: String,
    encryption: Encryption,
    auth: Option<Authentication>,
) -> Result<Vec<u8>, JsError> {
    let file_id = Uuid::parse_str(&file_id)
        .map_err(|err| JsError::new(&format!("Invalid passed to download_js: {err}")))?;
    let mut buffer = Vec::<u8>::new();
    let writer = BufWriter::new(&mut buffer);

    if let Err(e) = download(&file_id, &encryption, writer, auth.as_ref()).await {
        return Err(JsError::new(&format!("Download error: {e:?}")));
    };

    Ok(buffer)
}
