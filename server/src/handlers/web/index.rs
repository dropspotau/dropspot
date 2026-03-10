use askama::Template;
use axum::response::IntoResponse;

use crate::db::User;

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    is_logged_in: bool,
}

pub async fn handle_index(user: Option<User>) -> impl IntoResponse {
    let is_logged_in = user.is_some();

    let template = IndexTemplate { is_logged_in };
    HtmlTemplate(template)
}
