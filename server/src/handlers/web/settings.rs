use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};

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
