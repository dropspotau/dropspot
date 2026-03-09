use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
    db::{File, delete_files, get_file_by_id, get_files},
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "settings.html")]
struct SettingsTemplate {}

pub async fn handle_settings(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let template = SettingsTemplate {};
    HtmlTemplate(template)
}
