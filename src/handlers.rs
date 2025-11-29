use std::{
    io::{Read, Write},
    path::PathBuf,
};

use thiserror::Error;
use uuid::Uuid;

use crate::{download::Download, file::File, state::State, upload::Upload};

#[derive(Error, Debug)]
pub enum FileUploadError {
    #[error("Upload not found")]
    UploadNotFound,

    #[error("Failed to create file")]
    FileCreateError,

    #[error("Failed to write to file")]
    FileWriteError,
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

    #[error("Failed to open file")]
    FileOpenError,

    #[error("Failed to read file")]
    FileReadError,
}

pub fn handle_file_request_upload(state: &mut State) -> Uuid {
    let upload = Upload::generate();
    let upload_id = upload.id.clone();
    state.add_upload(upload);

    upload_id
}

pub fn handle_file_upload(
    state: &mut State,
    upload_id: Uuid,
    file_name: String,
    contents: Vec<u8>,
) -> Result<Uuid, FileUploadError> {
    let Some(upload) = state.get_upload_by_id(&upload_id) else {
        return Err(FileUploadError::UploadNotFound);
    };

    let path = PathBuf::from(&file_name);
    let size = contents.len();
    let file = File::new(file_name, upload.id.clone(), path.clone(), size);
    let file_id = file.id.clone();
    let file_path = file.get_path();
    state.add_file(file);

    let Ok(mut io_file) = std::fs::File::create(file_path) else {
        return Err(FileUploadError::FileCreateError);
    };

    if io_file.write(&contents).is_err() {
        // Don't save the file
        state.remove_files(&[file_id]);
        return Err(FileUploadError::FileWriteError);
    }

    Ok(file_id)
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

    let file_path = file.get_path();
    let Ok(mut io_file) = std::fs::File::open(file_path) else {
        return Err(FileDownloadError::FileOpenError);
    };

    let mut buffer = Vec::with_capacity(file.size);
    if io_file.read_to_end(&mut buffer).is_err() {
        return Err(FileDownloadError::FileReadError);
    }

    // Pretend that this would get a download URL link from S3 or Cloud Storage
    Ok(vec![])
}
