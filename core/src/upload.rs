use std::io::{BufReader, BufWriter};

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;
use crate::encryption::{Encryption, encrypt_file};
use crate::error::{ApiError, Error};
use crate::file::File;
use crate::integration::integration::Integration;
use crate::storage::StorageType;

#[derive(Serialize, Deserialize)]
pub struct CreateFileBody {
    pub name: String,
    pub size: i64,
    pub storage: StorageType,
    // TODO(alec): Allow expiry and download limits be specified at upload, rather than defaulting
    // to settings limits
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct UploadResult {
    pub file: File,
    pub encryption: Encryption,
}

#[wasm_bindgen]
pub async fn upload(
    name: String,
    contents: Vec<u8>,
    auth: Option<Authentication>,
    storage: StorageType,
) -> Result<UploadResult, Error> {
    let size = contents.len();
    let reader = BufReader::new(contents.as_slice());
    let mut encrypted_contents = Vec::<u8>::new();
    let writer = BufWriter::new(&mut encrypted_contents);

    let encryption = encrypt_file(reader, writer).map_err(|_e| Error::EncryptionError)?;

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
        .map_err(|_e| Error::NetworkError)?;

    if !file.status().is_success() {
        return Err(file
            .json::<ApiError>()
            .await
            .map(Error::ApiError)
            .map_err(|_e| Error::JsonError)?);
    }

    let file = file.json::<File>().await.map_err(|_e| Error::JsonError)?;

    // Upload the file body
    let mut headers = get_auth_headers(auth.as_ref());
    headers.insert("Content-Type", "application/octet-stream".parse().unwrap());

    let file_stream = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/upload/{}", file.id))
        .headers(headers)
        .body(encrypted_contents)
        .send()
        .await
        .map_err(|_e| Error::NetworkError)?;

    if let Err(_err) = file_stream.error_for_status() {
        return Err(Error::NetworkError);
    }

    Ok(UploadResult { file, encryption })
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PreviewUploadRequest {
    pub size: i64,
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

#[wasm_bindgen(js_name = previewUpload)]
pub async fn preview_upload(
    auth: Option<Authentication>,
    payload: PreviewUploadRequest,
) -> Result<PreviewUploadResult, Error> {
    let headers = get_auth_headers(auth.as_ref());

    let response = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/upload/preview"))
        .query(&payload)
        .headers(headers)
        .send()
        .await
        .map_err(|_e| Error::NetworkError)?;

    if !response.status().is_success() {
        return Err(response
            .json::<ApiError>()
            .await
            .map(Error::ApiError)
            .map_err(|_e| Error::JsonError)?);
    }

    let upload = response
        .json::<PreviewUploadResult>()
        .await
        .map_err(|_e| Error::JsonError)?;

    Ok(upload)
}
