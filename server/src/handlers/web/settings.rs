use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
    db::{File, User, delete_files, get_file_by_id, get_files, get_users},
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {
    users: Vec<User>,
}

pub async fn handle_settings(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pool = state.get_pool();
    let users = get_users(pool).await.unwrap();

    let template = SettingsTemplate { users };
    HtmlTemplate(template)
}
