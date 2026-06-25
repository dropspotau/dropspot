use sqlx::PgPool;
use uuid::Uuid;

pub struct Member {
    pub id: Uuid,
    pub is_admin: bool,
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
              id,
              is_admin
            from dropspot.member
            where organisation_id = $1 and user_id = $2
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
              id,
              is_admin
            from dropspot.member
            where organisation_id = $1
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
            insert into dropspot.member (organisation_id, user_id, is_admin)
            values ($1, $2, $3)
            returning id, is_admin
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
            update dropspot.member
            set is_admin = $2
            where id = $1
            returning id, is_admin
        "#,
        member_id,
        is_admin
    )
    .fetch_one(pool)
    .await
}
