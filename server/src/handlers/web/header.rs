use askama::Template;
use axum::response::IntoResponse;

use crate::db::User;

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "header.html")]
struct HeaderTemplate {
    name: Option<String>,
    is_logged_in: bool,
}

pub async fn handle_header(user: Option<User>) -> impl IntoResponse {
    let is_logged_in = user.is_some();
    let name = user.map(|u| u.get_name());

    let template = HeaderTemplate { name, is_logged_in };
    HtmlTemplate(template)
}
