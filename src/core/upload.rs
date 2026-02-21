use std::io::{BufRead, BufReader, BufWriter};

use crate::core::encryption::{Encryption, EncryptionError, encrypt_file};
use crate::server::handlers::{ApiFile, CreateFileBody};

use super::constants::ENDPOINT;

#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("Encryption error: {0}")]
    EncryptionError(EncryptionError),

    #[error("Upload error: {0}")]
    RequestError(reqwest::Error),
}

pub struct UploadResult {
    pub file: ApiFile,
    pub encryption: Encryption,
}

pub async fn upload(name: String, contents: Vec<u8>) -> Result<UploadResult, UploadError> {
    let size = contents.len();
    let reader = BufReader::new(contents.as_slice());
    let mut encrypted_contents = Vec::<u8>::new();
    let writer = BufWriter::new(&mut encrypted_contents);

    let encryption = encrypt_file(reader, writer).map_err(|e| UploadError::EncryptionError(e))?;

    // Request an upload
    let file = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/upload"))
        .header("Content-Type", "application/json")
        .json(&CreateFileBody {
            name,
            size: size as i64,
        })
        .send()
        .await
        .map_err(|e| UploadError::RequestError(e))?
        .json::<ApiFile>()
        .await
        .map_err(|e| UploadError::RequestError(e))?;

    // Upload the file body
    let file_stream = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/upload/{}/upload", file.id))
        .header("Content-Type", "application/octet-stream")
        .body(encrypted_contents)
        .send()
        .await
        .map_err(|e| UploadError::RequestError(e))?;

    if let Err(err) = file_stream.error_for_status() {
        return Err(UploadError::RequestError(err));
    }

    Ok(UploadResult { file, encryption })
}
