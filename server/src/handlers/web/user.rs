use std::sync::Arc;

use askama::Template;
use axum::{Form, extract::State, response::IntoResponse};
use serde::Deserialize;

use crate::{
    db::{User, create_user},
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {
    pub user: User,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn handle_login(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<LoginPayload>,
) -> impl IntoResponse {
    let pool = state.get_pool();
    let user = create_user(
        pool,
        &payload.first_name,
        &payload.last_name,
        &payload.email,
    )
    .await
    .unwrap();

    let template = LoginTemplate { user };
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "header.html")]
struct HeaderTemplate {}

pub async fn handle_header() -> impl IntoResponse {
    let template = HeaderTemplate {};
    HtmlTemplate(template)
}
