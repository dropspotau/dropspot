use std::path::PathBuf;

use thiserror::Error;
use uuid::Uuid;

use crate::{download::Download, file::File, state::State};

#[derive(Error, Debug)]
pub enum FileUploadError {
    #[error("Upload not found")]
    UploadNotFound,
}

#[derive(Error, Debug)]
pub enum FileDownloadError {
    #[error("File not found")]
    FileNotFound,

    #[error("File has expired")]
    FileExpired,

    #[error("Download URL not found")]
    DownloadNotFound,

    #[error("Download has expired")]
    DownloadExpired,
}

pub fn handle_file_upload(
    state: &mut State,
    upload_id: Uuid,
    file_name: String,
) -> Result<File, FileUploadError> {
    let Some(upload) = state.get_upload_by_id(&upload_id) else {
        return Err(FileUploadError::UploadNotFound);
    };

    let path = PathBuf::from(&file_name);
    Ok(File::new(file_name, upload.id.clone(), path))
}

pub fn handle_file_request_download(
    state: &mut State,
    file_id: Uuid,
) -> Result<Uuid, FileDownloadError> {
    let Some(file) = state.get_file_by_id(&file_id) else {
        return Err(FileDownloadError::FileNotFound);
    };

    let download = Download::generate(file.id.clone());
    let id = download.id.clone();
    state.add_download(download);

    Ok(id)
}

pub fn handle_file_download(
    state: &mut State,
    download_id: Uuid,
) -> Result<Vec<u8>, FileDownloadError> {
    let Some(download) = state.get_download_by_id(&download_id) else {
        return Err(FileDownloadError::DownloadNotFound);
    };

    if download.is_expired() {
        return Err(FileDownloadError::DownloadExpired);
    }

    let Some(file) = state.get_file_by_id(&download.file_id) else {
        return Err(FileDownloadError::FileNotFound);
    };

    if file.is_expired() {
        return Err(FileDownloadError::FileExpired);
    }

    // Pretend that this would get a download URL link from S3 or Cloud Storage
    Ok(vec![])
}
