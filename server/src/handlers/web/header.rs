use askama::Template;
use axum::response::IntoResponse;

use crate::db::User;

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "header.html")]
pub(crate) struct HeaderTemplate {
    pub name: Option<String>,
    pub is_logged_in: bool,
}

pub async fn handle_header(user: Option<User>) -> impl IntoResponse {
    let template = get_header_template(user.as_ref());
    HtmlTemplate(template)
}

pub(crate) fn get_header_template(user: Option<&User>) -> HeaderTemplate {
    let is_logged_in = user.is_some();
    let name = user.map(|u| u.get_name());

    HeaderTemplate { name, is_logged_in }
}
