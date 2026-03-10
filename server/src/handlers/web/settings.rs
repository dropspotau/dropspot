use askama::Template;
use axum::{extract::State, response::IntoResponse};

use crate::{
    db::{User, get_users},
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    users: Vec<User>,
}

pub async fn handle_settings(State(state): State<AppState>, user: User) -> impl IntoResponse {
    let pool = state.get_pool();
    let users = get_users(pool).await.unwrap();

    let template = SettingsTemplate { users };
    HtmlTemplate(template)
}
