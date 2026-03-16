mod gcp;
mod local;
mod s3;
mod storage;

pub use storage::{Storage, StorageType, get_storage};
