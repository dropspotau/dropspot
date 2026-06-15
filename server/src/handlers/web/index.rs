use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;

use crate::db::{User, get_onboarding_status};
use crate::state::AppState;

use super::header::{HeaderTemplate, get_header_template};
use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    header: HeaderTemplate,
    should_show_onboarding: bool,
}

pub async fn handle_index(State(state): State<AppState>, user: Option<User>) -> impl IntoResponse {
    let header = get_header_template(user.as_ref());

    let mut should_show_onboarding = false;

    if let Some(user) = user {
        let pool = state.get_pool();
        let onboarding = get_onboarding_status(pool, &user.id).await;
        should_show_onboarding = onboarding.unwrap_or(false);
    }

    let template = IndexTemplate {
        header,
        should_show_onboarding,
    };
    HtmlTemplate(template)
}
