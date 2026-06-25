use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct Member {
    pub id: Uuid,
    pub is_admin: bool,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub file_count: i32,
}

pub async fn get_organisation_member(
    pool: &PgPool,
    organisation_id: &Uuid,
    user_id: &Uuid,
) -> Result<Member, sqlx::Error> {
    sqlx::query_as!(
        Member,
        r#"
            select
              member.id,
              member.is_admin,
              users.id user_id,
              users.first_name,
              users.last_name,
              users.email,
              users.created_at,
              count(file.id)::int "file_count!"
            from dropspot.member member
            left join dropspot.users users
            on users.id = member.user_id
            left join dropspot.file file
            on file.created_by_id = users.id
            group by users.id, member.id
            having member.organisation_id = $1 and member.user_id = $2
        "#,
        organisation_id,
        user_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_organisation_members(
    pool: &PgPool,
    organisation_id: &Uuid,
) -> Result<Vec<Member>, sqlx::Error> {
    sqlx::query_as!(
        Member,
        r#"
            select
              member.id,
              member.is_admin,
              users.id user_id,
              users.first_name,
              users.last_name,
              users.email,
              users.created_at,
              count(file.id)::int "file_count!"
            from dropspot.member member
            left join dropspot.users users
            on users.id = member.user_id
            left join dropspot.file file
            on file.created_by_id = users.id
            group by users.id, member.id
            having member.organisation_id = $1
        "#,
        organisation_id,
    )
    .fetch_all(pool)
    .await
}

pub async fn create_organisation_member(
    pool: &PgPool,
    organisation_id: &Uuid,
    user_id: &Uuid,
    is_admin: bool,
) -> Result<Member, sqlx::Error> {
    sqlx::query_as!(
        Member,
        r#"
            with member_id as (
              insert into dropspot.member (organisation_id, user_id, is_admin)
              values ($1, $2, $3)
              returning id
            )
            select
              member.id,
              member.is_admin,
              users.id user_id,
              users.first_name,
              users.last_name,
              users.email,
              users.created_at,
              count(file.id)::int "file_count!"
            from dropspot.member member
            left join dropspot.users users
            on users.id = member.user_id
            left join dropspot.file file
            on file.created_by_id = users.id
            group by users.id, member.id
            having member.id = (select id from member_id limit 1)
        "#,
        organisation_id,
        user_id,
        is_admin
    )
    .fetch_one(pool)
    .await
}

pub async fn update_organisation_member(
    pool: &PgPool,
    member_id: &Uuid,
    is_admin: bool,
) -> Result<Member, sqlx::Error> {
    sqlx::query_as!(
        Member,
        r#"
            with member_id as (
              update dropspot.member
              set is_admin = $2
              where id = $1
              returning id
            )
            select
              member.id,
              member.is_admin,
              users.id user_id,
              users.first_name,
              users.last_name,
              users.email,
              users.created_at,
              count(file.id)::int "file_count!"
            from dropspot.member member
            left join dropspot.users users
            on users.id = member.user_id
            left join dropspot.file file
            on file.created_by_id = users.id
            group by users.id, member.id
            having member.id = (select id from member_id limit 1)
        "#,
        member_id,
        is_admin
    )
    .fetch_one(pool)
    .await
}
