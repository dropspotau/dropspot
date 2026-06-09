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
        get_users, update_organisation_settings, update_user_name,
    },
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    users: Vec<User>,
    file_expiry_minutes: i32,
    download_limit: i32,
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

    let template = SettingsTemplate {
        users,
        file_expiry_minutes,
        download_limit,
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
}

pub async fn handle_update_settings(
    State(state): State<AppState>,
    user: User,
    Form(payload): Form<UpdateSettingsPayload>,
) -> Response {
    let pool = state.get_pool();
    let organisation = get_organisation_for_user(pool, &user.id)
        .await
        .expect("Could not retrieve organisation for settings update");

    update_organisation_settings(
        pool,
        &organisation.id,
        payload.file_expiry_minutes,
        payload.download_limit,
    )
    .await
    .expect("Could not update organisation settings");

    let template = UpdateSettingsTemplate { success: true };
    HtmlTemplate(template).into_response()
}

#[derive(Template)]
#[template(path = "settings_user_update.html")]
struct UpdateUserTemplate {
    // The updated value
    success: bool,
}

#[derive(Deserialize)]
pub struct UpdateUserPayload {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
}

pub async fn handle_update_user(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Form(payload): Form<UpdateUserPayload>,
) -> Response {
    let is_same_user = id == user.id;

    // TOOD(alec): Admins should be able to update other users, not just themselves
    if !is_same_user {
        let template = UpdateUserTemplate { success: false };
        return (StatusCode::UNAUTHORIZED, HtmlTemplate(template)).into_response();
    }

    let pool = state.get_pool();

    let first_name = payload.first_name.unwrap_or(user.first_name.clone());
    let last_name = payload.last_name.unwrap_or(user.last_name.clone());
    let email = payload.email.unwrap_or(user.email.clone());

    if let Err(e) = update_user_name(pool, &id, &first_name, &last_name, &email).await {
        eprintln!("Failed to update user: {e}");
        let template = UpdateUserTemplate { success: false };
        return (StatusCode::INTERNAL_SERVER_ERROR, HtmlTemplate(template)).into_response();
    }

    let template = UpdateUserTemplate { success: true };
    HtmlTemplate(template).into_response()
}
