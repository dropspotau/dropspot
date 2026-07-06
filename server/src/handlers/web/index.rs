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
    should_show_disclaimer: bool,
}

pub async fn handle_index(State(state): State<AppState>, user: Option<User>) -> impl IntoResponse {
    let header = get_header_template(user.as_ref());

    let should_show_disclaimer = std::env::var("DROPSPOT_WEB_SHOW_DISCLAIMER")
        .unwrap_or("false".to_owned())
        .parse::<bool>();

    if should_show_disclaimer.is_err() {
        tracing::error!("Could not parse DROPSPOT_WEB_SHOW_DISCLAIMER as a boolean. Defaulting to false.");
    };

    let should_show_disclaimer = should_show_disclaimer.unwrap_or(false);

    let mut should_show_onboarding = false;

    if let Some(user) = user {
        let pool = state.get_pool();
        let onboarding = get_onboarding_status(pool, &user.id).await;
        should_show_onboarding = onboarding.map(|exists| !exists).unwrap_or(false);
    }

    let template = IndexTemplate {
        header,
        should_show_onboarding,
        should_show_disclaimer,
    };
    HtmlTemplate(template)
}
