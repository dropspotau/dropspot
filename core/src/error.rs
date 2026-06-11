use serde::{Deserialize, Serialize};
use tsify::Tsify;

use crate::{
    download::DownloadError, encryption::EncryptionError
    validation::FileValidationError,
};

// API error returned from the server
// NOTE(alec): This is almost identical to the ApiError in the server crate, but is only a separate struct here because
// the core package doesn't need to be aware of how to implement IntoResponse for Axum
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, thiserror::Error, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum Error {
    #[error("Upload error")]
    UploadError(ApiError),

    #[error("Download error: {0}")]
    DownloadError(DownloadError),

    #[error("Encryption error: {0}")]
    EncryptionError(EncryptionError),

    #[error("Validation error: {0}")]
    ValidationError(FileValidationError),

    #[error("User error")]
    UserError(ApiError),

    #[error("Network error")]
    NetworkError(reqwest::Error),

    #[error("JSON parsing error")]
    JsonError(reqwest::Error),
}
