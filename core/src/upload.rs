use std::io::{BufReader, BufWriter};

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;
use crate::encryption::{Encryption, EncryptionError, encrypt_file};
use crate::file::File;
use crate::storage::StorageType;

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("Encryption error: {0}")]
    EncryptionError(EncryptionError),

    #[error("Upload error: {0}")]
    RequestError(reqwest::Error),
}

#[derive(Serialize, Deserialize)]
pub struct CreateFileBody {
    pub name: String,
    pub size: i64,
    pub storage: StorageType,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct UploadResult {
    pub file: File,
    pub encryption: Encryption,
}

pub async fn upload(
    name: String,
    contents: Vec<u8>,
    auth: Option<Authentication>,
    storage: StorageType,
) -> Result<UploadResult, UploadError> {
    let size = contents.len();
    let reader = BufReader::new(contents.as_slice());
    let mut encrypted_contents = Vec::<u8>::new();
    let writer = BufWriter::new(&mut encrypted_contents);

    let encryption = encrypt_file(reader, writer).map_err(|e| UploadError::EncryptionError(e))?;

    let mut headers = get_auth_headers(auth.as_ref());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // Request an upload
    let file = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/upload"))
        .headers(headers)
        .json(&CreateFileBody {
            name,
            size: size as i64,
            storage,
        })
        .send()
        .await
        .map_err(|e| UploadError::RequestError(e))?
        .json::<File>()
        .await
        .map_err(|e| UploadError::RequestError(e))?;

    // Upload the file body
    let mut headers = get_auth_headers(auth.as_ref());
    headers.insert("Content-Type", "application/octet-stream".parse().unwrap());

    let file_stream = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/upload/{}/upload", file.id))
        .headers(headers)
        .body(encrypted_contents)
        .send()
        .await
        .map_err(|e| UploadError::RequestError(e))?;

    if let Err(err) = file_stream.error_for_status() {
        return Err(UploadError::RequestError(err));
    }

    Ok(UploadResult { file, encryption })
}

#[wasm_bindgen]
// #[cfg(target_arch = "wasm32")]
pub async fn upload_js(
    name: String,
    contents: Vec<u8>,
    auth: Option<Authentication>,
    storage: StorageType,
) -> Result<UploadResult, JsError> {
    let upload = upload(name, contents, auth, storage).await;

    if let Err(e) = upload {
        return Err(JsError::new(&format!("{e:?}")));
    };

    let upload = upload.unwrap();
    Ok(upload)
}
