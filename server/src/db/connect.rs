use sqlx::PgPool;

use super::error::DatabaseError;

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
