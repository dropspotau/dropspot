use askama::Template;
use axum::response::IntoResponse;

use crate::db::User;

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub async fn handle_index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "header.html")]
struct HeaderTemplate {
    name: Option<String>,
}

pub async fn handle_header(user: Option<User>) -> impl IntoResponse {
    let name = user.map(|u| u.get_name());

    let template = HeaderTemplate { name };
    HtmlTemplate(template)
}
