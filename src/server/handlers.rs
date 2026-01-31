use std::{
    io::{Read, Write},
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;
use uuid::Uuid;

use super::{
    db::{
        Download, File, Upload, create_download, create_file, create_upload, delete_files,
        get_download_by_id, get_file_by_id, get_upload_by_id,
    },
    state::State,
};

#[derive(Error, Debug)]
pub enum FileUploadError {
    #[error("Upload not found")]
    UploadNotFound,

    #[error("Error writing file to database: {0}")]
    FileDatabaseCreateError(sqlx::Error),

    #[error("Error deleting file from database: {0}")]
    FileDatabaseDeleteError(sqlx::Error),

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

    #[error("Error creating download record: {0}")]
    DownloadCreateError(sqlx::Error),

    #[error("Download URL not found")]
    DownloadNotFound,

    #[error("Download has expired")]
    DownloadExpired,

    #[error("Failed to open file")]
    FileOpenError,

    #[error("Failed to read file")]
    FileReadError,
}


#[derive(Serialize)]
pub struct ApiUpload {
    id: Uuid
}

impl From<Upload> for ApiUpload {
    fn from(upload: Upload) -> Self {
        Self {
            id: upload.id
        }
    }
}

// TODO(alec): Make this into an Axum view
pub async fn handle_file_request_upload(state: &mut State) -> Result<ApiUpload, sqlx::Error> {
    create_upload(state.get_pool()).await.map(Into::into)
}

#[derive(Serialize)]
pub struct ApiFile {
    id: Uuid,
    name: String,
    size: i64,
}

impl From<File> for ApiFile {
    fn from(file: File) -> Self {
        Self {
            id: file.id,
            name: file.name,
            size: file.size,
        }
    }
}

// TODO(alec): Make this into an Axum view
pub async fn handle_file_upload(
    state: &mut State,
    upload_id: &Uuid,
    file_name: String,
    contents: Vec<u8>,
) -> Result<ApiFile, FileUploadError> {
    let pool = state.get_pool();

    let Ok(upload) = get_upload_by_id(pool, &upload_id).await else {
        return Err(FileUploadError::UploadNotFound);
    };

    let path = PathBuf::from(&file_name);
    let size = contents.len();

    let pool = state.get_pool();
    let file = create_file(
        pool,
        file_name,
        &upload.id,
        path.to_str().unwrap().to_owned(),
        size as i64,
    )
    .await;

    if let Err(e) = file {
        return Err(FileUploadError::FileDatabaseCreateError(e));
    }

    let file = file.unwrap();

    let Ok(mut io_file) = std::fs::File::create(file.get_path()) else {
        return Err(FileUploadError::FileCreateError);
    };

    if io_file.write(&contents).is_err() {
        // Don't save the file
        if let Err(e) = delete_files(pool, &[file.id]).await {
            return Err(FileUploadError::FileDatabaseCreateError(e));
        };

        return Err(FileUploadError::FileWriteError);
    }

    Ok(file.into())
}

#[derive(Serialize)]
pub struct ApiDownload {
    id: Uuid,
    url: String,
    expires_at: DateTime<Utc>,
}

impl From<Download> for ApiDownload {
    fn from(download: Download) -> Self {
        Self {
            id: download.id,
            url: download.url,
            expires_at: download.expires_at,
        }
    }
}

// TODO(alec): Make this into an Axum view
pub async fn handle_file_request_download(
    state: &mut State,
    file_id: Uuid,
) -> Result<ApiDownload, FileDownloadError> {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &file_id).await else {
        return Err(FileDownloadError::FileNotFound);
    };

    if file.is_expired() {
        return Err(FileDownloadError::FileExpired);
    }

    let download = create_download(pool, &file.id).await;

    if let Err(e) = download {
        return Err(FileDownloadError::DownloadCreateError(e));
    };

    Ok(download.unwrap().into())
}

// TODO(alec): Make this into an Axum view
pub async fn handle_file_download(
    state: &mut State,
    download_id: Uuid,
) -> Result<impl Iterator<Item = u8> + use<>, FileDownloadError> {
    let pool = state.get_pool();

    let Ok(download) = get_download_by_id(pool, &download_id).await else {
        return Err(FileDownloadError::DownloadNotFound);
    };

    if download.is_expired() {
        return Err(FileDownloadError::DownloadExpired);
    }

    let Ok(file) = get_file_by_id(pool, &download.file_id).await else {
        return Err(FileDownloadError::FileNotFound);
    };

    let file_path = file.get_path();
    let Ok(mut io_file) = std::fs::File::open(file_path) else {
        return Err(FileDownloadError::FileOpenError);
    };

    // TODO (alec): Don't read the whole file into memory
    let mut buffer = Vec::with_capacity(file.size as usize);
    if io_file.read_to_end(&mut buffer).is_err() {
        return Err(FileDownloadError::FileReadError);
    }

    // Pretend that this would get a download URL link from S3 or Cloud Storage
    Ok(buffer.into_iter())
}
