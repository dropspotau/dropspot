use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::types::Id;

pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by_id: Option<Uuid>,
    pub organisation_id: Uuid,
    pub is_admin: bool,
    pub file_count: i32,
}

impl User {
    pub fn get_name(&self) -> String {
        if !self.first_name.is_empty() && !self.last_name.is_empty() {
            return format!("{} {}", self.first_name, self.last_name);
        }

        if !self.first_name.is_empty() {
            return self.first_name.clone();
        }

        self.email.clone()
    }

    pub fn created_at_slash_date(&self) -> String {
        format!("{}", self.created_at.format("%d/%m/%Y"))
    }
}

pub async fn get_users(pool: &PgPool, organisation_id: &Uuid) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            select
              users.id,
              users.first_name,
              users.last_name,
              users.email,
              users.created_at,
              users.updated_at,
              users.updated_by_id,
              users.organisation_id,
              users.is_admin,
              count(file.id)::int "file_count!"
            from dropspot.users users
            left join dropspot.file file
            on file.created_by_id = users.id
            group by users.id
            having users.organisation_id = $1
            order by users.created_at asc
        "#,
        organisation_id
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
              users.updated_at,
              users.updated_by_id,
              users.organisation_id,
              users.is_admin,
              count(file.id)::int "file_count!"
            from dropspot.users users
            left join dropspot.file file
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
              users.updated_at,
              users.updated_by_id,
              users.organisation_id,
              users.is_admin,
              count(file.id)::int "file_count!"
            from dropspot.users users
            left join dropspot.file file
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

pub async fn get_user_password(pool: &PgPool, user_id: &Uuid) -> Result<String, sqlx::Error> {
    let password = sqlx::query_as!(
        Password,
        r#"
            select
              password
            from dropspot.password
            where user_id = $1
            limit 1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(password.password)
}

pub async fn create_user(
    pool: &PgPool,
    email: &str,
    first_name: &str,
    last_name: &str,
    organisation_id: &Uuid,
    is_admin: bool,
    password: &str,
) -> Result<User, sqlx::Error> {
    let mut transaction = pool.begin().await?;
    let user_id = sqlx::query_as!(
        Id,
        r#"
            with user_id as (
                insert into dropspot.users (email, first_name, last_name, organisation_id, is_admin)
                values ($1, $2, $3, $4, $5)
                returning id
            )
            insert into dropspot.password (user_id, password)
            values ((select id from user_id limit 1), $6)
            returning user_id id
        "#,
        email,
        first_name,
        last_name,
        organisation_id,
        is_admin,
        password
    )
    .fetch_one(&mut *transaction)
    .await?;

    transaction.commit().await?;

    get_user_by_id(pool, &user_id.id).await
}

pub async fn update_user(
    pool: &PgPool,
    id: &Uuid,
    first_name: &str,
    last_name: &str,
    email: &str,
    is_admin: bool,
    updated_by_id: &Uuid,
) -> Result<User, sqlx::Error> {
    let id = sqlx::query_as!(
        Id,
        r#"
            update dropspot.users
            set first_name = $2, last_name = $3, email = $4, is_admin = $5, updated_at = now(), updated_by_id = $6
            where id = $1
            returning id
        "#,
        id,
        first_name,
        last_name,
        email,
        is_admin,
        updated_by_id
    )
    .fetch_one(pool)
    .await?;

    get_user_by_id(pool, &id.id).await
}
