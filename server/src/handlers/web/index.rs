use askama::Template;
use axum::response::IntoResponse;

use crate::db::User;

use super::header::{HeaderTemplate, get_header_template};
use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    is_logged_in: bool,

    header: HeaderTemplate,
}

pub async fn handle_index(user: Option<User>) -> impl IntoResponse {
    let is_logged_in = user.is_some();
    let header = get_header_template(user.as_ref());

    let template = IndexTemplate {
        is_logged_in,
        header,
    };
    HtmlTemplate(template)
}
