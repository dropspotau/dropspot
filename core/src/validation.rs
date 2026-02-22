use std::fs::{File, metadata};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileValidationError {
    #[error("Filesystem entry does not exist")]
    EntryNotFound,

    #[error("Cannot upload directories")]
    IsDirectory,

    #[error("Failed to open file")]
    ErrorOpeningFile(std::io::Error),
}

pub fn validate_file(file_path: &str) -> Result<File, FileValidationError> {
    let Ok(metadata) = metadata(file_path) else {
        return Err(FileValidationError::EntryNotFound);
    };

    if metadata.is_dir() {
        return Err(FileValidationError::IsDirectory);
    }

    std::fs::File::open(file_path).map_err(FileValidationError::ErrorOpeningFile)
}
