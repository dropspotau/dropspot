use askama::Template;
use axum::{
    Form,
    extract::State,
    response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::{
    db::{User, get_users},
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    users: Vec<User>,
    file_expiry_minutes: i32,
    download_limit: i32,
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
    let users = get_users(pool).await.unwrap();

    let file_expiry_minutes = 60;
    let download_limit = 100;

    let template = SettingsTemplate {
        users,
        file_expiry_minutes,
        download_limit,
    };
    HtmlTemplate(template).into_response()
}

#[derive(Template)]
#[template(path = "settings_update.html")]
struct UpdateSettingsTemplate {
    success: bool,
}

#[derive(Deserialize)]
pub struct UpdateSettingsPayload {
    file_expiry_minutes: Option<i32>,
    download_limit: Option<i32>,
}

pub async fn handle_update_settings(
    State(state): State<AppState>,
    user: User,
    Form(payload): Form<UpdateSettingsPayload>,
) -> Response {
    if let Some(ref file_expiry_minutes) = payload.file_expiry_minutes {
        println!("file_expiry_minutes: {file_expiry_minutes}");
    }

    if let Some(ref download_limit) = payload.download_limit {
        println!("download_limit: {download_limit}");
    }

    let template = UpdateSettingsTemplate { success: true };
    HtmlTemplate(template).into_response()
}
