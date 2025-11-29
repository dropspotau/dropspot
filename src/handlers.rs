use std::path::PathBuf;

use thiserror::Error;
use uuid::Uuid;

use crate::{file::File, state::State};

#[derive(Error, Debug)]
pub enum FileUploadError {
    #[error("Upload not found")]
    UploadNotFound,
}

pub fn handle_file(
    state: &mut State,
    upload_key: Uuid,
    file_name: String,
) -> Result<File, FileUploadError> {
    let Some(upload) = state.uploads.iter().find(|upload| upload.key == upload_key) else {
        return Err(FileUploadError::UploadNotFound);
    };

    let path = PathBuf::from(&file_name);
    Ok(File::new(file_name, upload.key.clone(), path))
}
