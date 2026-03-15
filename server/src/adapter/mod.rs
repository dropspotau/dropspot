mod adapter;
mod gcp;
mod local;
mod s3;

pub use adapter::{Adapter, AdapterType, get_adapter};
