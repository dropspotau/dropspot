use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("`DATABASE_URL` environent variable is not set")]
    MissingDatabaseUrlError,

    #[error("Database error: {0}")]
    ConnectionError(sqlx::Error),
}
