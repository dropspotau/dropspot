use sqlx::{PgPool, migrate};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("`DATABASE_URL` environent variable is not set")]
    MissingDatabaseUrlError,

    #[error("Database error: {0}")]
    ConnectionError(sqlx::Error),

    #[error("Migration error: {0}")]
    MigrationError(sqlx::migrate::MigrateError),
}

/// Returns a Postgres database connection pool.
pub async fn connect() -> Result<PgPool, DatabaseError> {
    let Ok(db_url) = std::env::var("DATABASE_URL") else {
        return Err(DatabaseError::MissingDatabaseUrlError);
    };

    let db = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await;

    if let Err(e) = db {
        return Err(DatabaseError::ConnectionError(e));
    }

    Ok(db.unwrap())
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), DatabaseError> {
    const MIGRATION_PATH: &'static str = "./migrations";
    migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| DatabaseError::MigrationError(e))
}
