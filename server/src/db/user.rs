use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::db::organisation::{DEFAULT_ORGANISATION_NAME, create_organisation, get_organisation};

use super::types::Id;

pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub file_count: i32,
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            select
              users.id,
              users.first_name,
              users.last_name,
              users.email,
              users.created_at,
              count(file.id)::int "file_count!"
            from users
            left join file
            on file.created_by_id = users.id
            group by users.id
            order by users.created_at asc
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_user_by_id(pool: &PgPool, id: &Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            select
              users.id,
              users.first_name,
              users.last_name,
              users.email,
              users.created_at,
              count(file.id)::int "file_count!"
            from users
            left join file
            on file.created_by_id = users.id
            group by users.id
            having users.id = $1
            order by users.created_at asc
            limit 1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            select
              users.id,
              users.first_name,
              users.last_name,
              users.email,
              users.created_at,
              count(file.id)::int "file_count!"
            from users
            left join file
            on file.created_by_id = users.id
            group by users.id
            having users.email = $1
            order by users.created_at asc
            limit 1
        "#,
        email
    )
    .fetch_one(pool)
    .await
}

struct Password {
    password: String,
}

pub async fn get_user_password(pool: &PgPool, id: &Uuid) -> Result<String, sqlx::Error> {
    let password = sqlx::query_as!(
        Password,
        r#"
            select
              users.password
            from users
            where users.id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(password.password)
}

pub async fn create_user(
    pool: &PgPool,
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
) -> Result<User, sqlx::Error> {
    let mut organisation = get_organisation(pool).await;

    if organisation.is_err() {
        organisation = create_organisation(pool, DEFAULT_ORGANISATION_NAME).await;
    }

    if let Err(e) = organisation {
        return Err(e);
    }

    let organisation = organisation.unwrap();

    let id = sqlx::query_as!(
        Id,
        r#"
            with inserted_user as (
                insert into users (first_name, last_name, email, password)
                values ($1, $2, $3, $4)
                returning id
            )
            insert into member (organisation_id, user_id)
            values ($5, (select id from inserted_user))
            returning user_id id
        "#,
        first_name,
        last_name,
        email,
        password,
        &organisation.id
    )
    .fetch_one(pool)
    .await?;

    get_user_by_id(pool, &id.id).await
}
