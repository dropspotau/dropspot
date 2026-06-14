use askama::Template;
use axum::response::IntoResponse;

use crate::db::User;

use super::header::{HeaderTemplate, get_header_template};
use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    header: HeaderTemplate,
    should_show_onboarding: bool,
}

pub async fn handle_index(user: Option<User>) -> impl IntoResponse {
    let header = get_header_template(user.as_ref());

    let should_show_onboarding = user.is_some();

    let template = IndexTemplate {
        header,
        should_show_onboarding,
    };
    HtmlTemplate(template)
}
