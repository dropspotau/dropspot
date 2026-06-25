use askama::Template;
use axum::{
    Form,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    db::{
        Integration, Member, User, get_integrations, get_organisation_for_user,
        get_organisation_member, get_organisation_settings, get_users, update_organisation_member,
        update_organisation_settings, update_user_name,
    },
    handlers::utils::get_organisation_from_request_user,
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    members: Vec<Member>,
    file_expiry_minutes: i32,
    download_limit: i32,
    allow_external_uploads: bool,
    allow_external_downloads: bool,
    max_file_size_mb: i32,
    current_user_id: Uuid,
    integrations: Vec<Integration>,
}

#[derive(Template)]
#[template(path = "settings_unauthed.html")]
struct SettingsUnAuthedTemplate {}

pub async fn handle_settings(State(state): State<AppState>, user: Option<User>) -> Response {
    if user.is_none() {
        let template = SettingsUnAuthedTemplate {};
        return HtmlTemplate(template).into_response();
    }

    let pool = state.get_pool();
    let current_user = user.unwrap();
    let users = get_users(pool).await.unwrap();

    let organisation = get_organisation_for_user(pool, &current_user.id)
        .await
        .expect("Could not retrieve organisation for user");
    let integrations = get_integrations(pool, &organisation.id).await.unwrap();
    let settings = get_organisation_settings(pool, &organisation.id)
        .await
        .expect("Could not retrieve settings for organisation");

    let file_expiry_minutes = settings.default_file_expiry_minutes;
    let download_limit = settings.default_download_limit;
    let allow_external_uploads = settings.allow_external_uploads;
    let allow_external_downloads = settings.allow_external_downloads;
    let max_file_size_mb = settings.max_file_size_mb;

    let template = SettingsTemplate {
        members: users,
        file_expiry_minutes,
        download_limit,
        allow_external_uploads,
        allow_external_downloads,
        max_file_size_mb,
        current_user_id: current_user.id,
        integrations,
    };
    HtmlTemplate(template).into_response()
}

#[derive(Template)]
#[template(path = "settings_update.html")]
pub(crate) struct UpdateSettingsTemplate {
    pub success: bool,
}

#[derive(Deserialize)]
pub struct UpdateSettingsPayload {
    file_expiry_minutes: i32,
    download_limit: i32,
    #[serde(default)]
    allow_external_uploads: bool,
    #[serde(default)]
    allow_external_downloads: bool,
    max_file_size_mb: i32,
}

impl UpdateSettingsPayload {
    pub fn validate(&self) -> bool {
        if self.max_file_size_mb <= 0 {
            return false;
        }

        self.file_expiry_minutes > 0 && self.download_limit > 0
    }
}

pub async fn handle_update_settings(
    State(state): State<AppState>,
    user: User,
    Form(payload): Form<UpdateSettingsPayload>,
) -> Response {
    let pool = state.get_pool();
    let Ok(organisation) = get_organisation_for_user(pool, &user.id).await else {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::NOT_FOUND, HtmlTemplate(template)).into_response();
    };
    let Ok(member) = get_organisation_member(pool, &organisation.id, &user.id).await else {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::NOT_FOUND, HtmlTemplate(template)).into_response();
    };

    let can_edit = member.is_admin;
    if !can_edit {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::FORBIDDEN, HtmlTemplate(template)).into_response();
    }

    if !payload.validate() {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::BAD_REQUEST, HtmlTemplate(template)).into_response();
    }

    if update_organisation_settings(
        pool,
        &organisation.id,
        payload.file_expiry_minutes,
        payload.download_limit,
        payload.allow_external_uploads,
        payload.allow_external_downloads,
        payload.max_file_size_mb,
    )
    .await
    .is_err()
    {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::INTERNAL_SERVER_ERROR, HtmlTemplate(template)).into_response();
    }

    let template = UpdateSettingsTemplate { success: true };
    HtmlTemplate(template).into_response()
}

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    // User fields
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,

    // Member fields
    is_admin: Option<bool>,
}

pub async fn handle_update_user(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Form(payload): Form<UpdateUserPayload>,
) -> Response {
    let pool = state.get_pool();

    let Ok(organisation) = get_organisation_from_request_user(pool, Some(&user)).await else {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::UNAUTHORIZED, HtmlTemplate(template)).into_response();
    };
    let Ok(user_member) = get_organisation_member(pool, &organisation.id, &user.id).await else {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::UNAUTHORIZED, HtmlTemplate(template)).into_response();
    };
    let Ok(member) = get_organisation_member(pool, &organisation.id, &id).await else {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::UNAUTHORIZED, HtmlTemplate(template)).into_response();
    };

    let is_same_member = user_member.id == member.id;
    let can_update = is_same_member || user_member.is_admin;

    if !can_update {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::UNAUTHORIZED, HtmlTemplate(template)).into_response();
    }

    let first_name = payload.first_name.unwrap_or(user.first_name.clone());
    let last_name = payload.last_name.unwrap_or(user.last_name.clone());
    let email = payload.email.unwrap_or(user.email.clone());

    if let Err(e) = update_user_name(pool, &id, &first_name, &last_name, &email).await {
        tracing::error!("Failed to update user {id}: {e}");
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::INTERNAL_SERVER_ERROR, HtmlTemplate(template)).into_response();
    }

    if let Some(is_admin) = payload.is_admin {
        let can_update_admin_status = user_member.is_admin;

        if !can_update_admin_status {
            let template = UpdateSettingsTemplate { success: false };
            return (StatusCode::BAD_REQUEST, HtmlTemplate(template)).into_response();
        }

        if let Err(e) = update_organisation_member(pool, &member.id, is_admin).await {
            tracing::error!("Failed to update member {}: {e}", member.id);
            let template = UpdateSettingsTemplate { success: false };
            return (StatusCode::BAD_REQUEST, HtmlTemplate(template)).into_response();
        }
    }

    let template = UpdateSettingsTemplate { success: true };
    HtmlTemplate(template).into_response()
}
