use reqwest::StatusCode;
use thiserror::Error;

use crate::types::ApiError;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("`DROPSPOT_DATABASE_URL` environent variable is not set")]
    MissingDatabaseUrlError,

    #[error("Database error: {0}")]
    ConnectionError(sqlx::Error),
}

impl Into<ApiError> for DatabaseError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
