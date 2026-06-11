use std::io::{BufReader, BufWriter};

use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;
use crate::encryption::{Encryption, EncryptionError, encrypt_file};
use crate::file::File;
use crate::integration::integration::Integration;
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
    // TODO(alec): Allow expiry and download limits be specified at upload, rather than defaulting
    // to settings
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
    auth: Option<&Authentication>,
    storage: StorageType,
) -> Result<UploadResult, UploadError> {
    let size = contents.len();
    let reader = BufReader::new(contents.as_slice());
    let mut encrypted_contents = Vec::<u8>::new();
    let writer = BufWriter::new(&mut encrypted_contents);

    let encryption = encrypt_file(reader, writer).map_err(|e| UploadError::EncryptionError(e))?;

    let mut headers = get_auth_headers(auth);
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
    let mut headers = get_auth_headers(auth);
    headers.insert("Content-Type", "application/octet-stream".parse().unwrap());

    let file_stream = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/upload/{}", file.id))
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

#[wasm_bindgen(js_name = upload)]
// #[cfg(target_arch = "wasm32")]
pub async fn upload_js(
    name: String,
    contents: Vec<u8>,
    auth: Option<Authentication>,
    storage: StorageType,
) -> Result<UploadResult, JsError> {
    let upload = upload(name, contents, auth.as_ref(), storage).await;

    if let Err(e) = upload {
        return Err(JsError::new(&format!("{e}")));
    };

    let upload = upload.unwrap();
    Ok(upload)
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PreviewUploadRequest {
    pub size: usize,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PreviewUploadResult {
    // Whether the user is allowed to upload the file
    pub can_upload: bool,
    // Whether the user is unable to upload any more files
    pub is_at_file_limit: bool,
    // Whether the file exceeds the upload size limit
    pub file_too_large: bool,
    // The integrations available to use with this upload
    pub integrations: Vec<Integration>,
}

pub async fn preview_upload(
    auth: Option<&Authentication>,
    payload: PreviewUploadRequest,
) -> Result<PreviewUploadResult, reqwest::Error> {
    let headers = get_auth_headers(auth);

    reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/upload/preview"))
        .query(&payload)
        .headers(headers)
        .send()
        .await?
        .json::<PreviewUploadResult>()
        .await
}

#[wasm_bindgen(js_name = previewUpload)]
pub async fn preview_upload_js(
    auth: Option<Authentication>,
    payload: PreviewUploadRequest,
) -> Result<PreviewUploadResult, JsError> {
    preview_upload(auth.as_ref(), payload)
        .map_err(|e| JsError::new(&format!("{e}")))
        .await
}
