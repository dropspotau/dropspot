use crate::{
    download::DownloadError, encryption::EncryptionError, upload::UploadError, user::UserError,
    validation::FileValidationError,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Upload error: {0}")]
    UploadError(UploadError),

    #[error("Download error: {0}")]
    DownloadError(DownloadError),

    #[error("Encryption error: {0}")]
    EncryptionError(EncryptionError),

    #[error("Validation error: {0}")]
    ValidationError(FileValidationError),

    #[error("User error: {0}")]
    UserError(UserError),
}
