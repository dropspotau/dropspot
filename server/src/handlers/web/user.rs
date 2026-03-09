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

