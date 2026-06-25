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
        Integration, User, get_integrations, get_organisation_for_user, get_organisation_settings,
        get_user_by_id, get_users, update_organisation_settings, update_user,
    },
    state::AppState,
    types::ApiError,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    users: Vec<User>,
    file_expiry_minutes: i32,
    download_limit: i32,
    allow_external_uploads: bool,
    allow_external_downloads: bool,
    max_file_size_mb: i32,
    current_user: User,
    is_only_admin: bool,
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
    let user = user.unwrap();
    let Ok(organisation) = get_organisation_for_user(pool, &user.id).await else {
        return ApiError::new(
            "Could not find organisation".to_owned(),
            StatusCode::FORBIDDEN,
        )
        .into_response();
    };

    let users = get_users(pool, &organisation.id).await.unwrap();
    let integrations = get_integrations(pool, &organisation.id).await.unwrap();
    let settings = get_organisation_settings(pool, &organisation.id)
        .await
        .expect("Could not retrieve settings for organisation");

    let file_expiry_minutes = settings.default_file_expiry_minutes;
    let download_limit = settings.default_download_limit;
    let allow_external_uploads = settings.allow_external_uploads;
    let allow_external_downloads = settings.allow_external_downloads;
    let max_file_size_mb = settings.max_file_size_mb;
    let is_only_admin = users
        .iter()
        .filter(|u| u.id != user.id && u.is_admin)
        .collect::<Vec<&User>>()
        .is_empty();

    let template = SettingsTemplate {
        users,
        file_expiry_minutes,
        download_limit,
        allow_external_uploads,
        allow_external_downloads,
        max_file_size_mb,
        current_user: user,
        is_only_admin,
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

    let can_edit = user.is_admin;
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
    first_name: String,
    last_name: String,
    email: String,
    is_admin: bool,
}

pub async fn handle_update_user(
    State(state): State<AppState>,
    current_user: User,
    Path(id): Path<Uuid>,
    Form(payload): Form<UpdateUserPayload>,
) -> Response {
    let pool = state.get_pool();

    let Ok(user) = get_user_by_id(pool, &id).await else {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::NOT_FOUND, HtmlTemplate(template)).into_response();
    };
    let Ok(all_users) = get_users(pool, &user.organisation_id).await else {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::NOT_FOUND, HtmlTemplate(template)).into_response();
    };

    let is_different_organisation = current_user.organisation_id != user.organisation_id;
    if is_different_organisation {
        // Can't update users in different organisations
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::NOT_FOUND, HtmlTemplate(template)).into_response();
    }

    let is_same_member = current_user.id == user.id;
    let can_update = is_same_member || current_user.is_admin;

    if !can_update {
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::UNAUTHORIZED, HtmlTemplate(template)).into_response();
    }

    // Prevent the last admin from demoting themselves
    let is_only_admin = all_users
        .into_iter()
        .filter(|u| u.id != current_user.id && u.is_admin)
        .collect::<Vec<User>>()
        .is_empty();
    let can_update_admin_status = match is_only_admin {
        // Only allow when promoting someone else
        true => current_user.is_admin && user.id != current_user.id && payload.is_admin,
        // Only allow when changing someone else
        false => current_user.is_admin && user.id != current_user.id,
    };

    if !can_update_admin_status && payload.is_admin != user.is_admin {
        // Admin status attempted to be changed when it shouldn't have been
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::BAD_REQUEST, HtmlTemplate(template)).into_response();
    }

    if let Err(e) = update_user(
        pool,
        &id,
        &payload.first_name,
        &payload.last_name,
        &payload.email,
        payload.is_admin,
        &current_user.id,
    )
    .await
    {
        tracing::error!("Failed to update user {id}: {e}");
        let template = UpdateSettingsTemplate { success: false };
        return (StatusCode::INTERNAL_SERVER_ERROR, HtmlTemplate(template)).into_response();
    }

    let template = UpdateSettingsTemplate { success: true };
    HtmlTemplate(template).into_response()
}
