use serde::{Deserialize, Serialize};
use tsify::Tsify;

// API error returned from the server
// NOTE(alec): This is almost identical to the ApiError in the server crate, but is only a separate struct here because
// the core package doesn't need to be aware of how to implement IntoResponse for Axum
#[derive(Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ApiError {
    pub message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum EncryptionError {
    #[error("Cipher error")]
    CipherError(aes_gcm::Error),
}

#[derive(Serialize, Deserialize, Debug, thiserror::Error, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(tag = "type")]
pub enum Error {
    #[error("Encryption error")]
    EncryptionError,

    #[error("Validation error")]
    ValidationError,

    #[error("Network error")]
    NetworkError,

    #[error("JSON parsing error")]
    JsonError,

    #[error("API error")]
    ApiError(ApiError),

    #[error("WASM conversion error")]
    WasmConversionError,
}
