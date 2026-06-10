use sqlx::PgPool;

use crate::db::{Organisation, User, get_default_organisation, get_organisation_for_user};

pub async fn get_organisation_from_request_user(
    pool: &PgPool,
    user: Option<&User>,
) -> Result<Organisation, sqlx::Error> {
    match user {
        Some(u) => get_organisation_for_user(pool, &u.id).await,
        None => get_default_organisation(pool).await,
    }
}
