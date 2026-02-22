use askama::Template;
use axum::response::IntoResponse;

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
struct HeaderTemplate {}

pub async fn handle_header() -> impl IntoResponse {
    let template = HeaderTemplate {};
    HtmlTemplate(template)
}
