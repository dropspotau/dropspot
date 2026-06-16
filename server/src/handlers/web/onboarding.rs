use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

use crate::{
    db::{User, record_onboarding_completion},
    handlers::web::template::HtmlTemplate,
    state::AppState,
};

#[derive(Template)]
#[template(path = "onboarding.html")]
struct OnboardingTemplate {}

pub async fn handle_onboarding() -> Response {
    let template = OnboardingTemplate {};
    HtmlTemplate(template).into_response()
}

#[derive(Template)]
#[template(path = "onboarding_record.html")]
struct RecordOnboardingTemplate {
    success: bool,
}

pub async fn handle_record_onboarding(State(state): State<AppState>, user: User) -> Response {
    let pool = state.get_pool();
    let onboarding = record_onboarding_completion(pool, &user.id).await;
    let success = onboarding.is_ok();

    let template = RecordOnboardingTemplate { success };

    if let Err(e) = onboarding {
        tracing::error!("Onboarding error: {e} user_id={}", &user.id);
        return (StatusCode::BAD_REQUEST, HtmlTemplate(template)).into_response();
    }

    HtmlTemplate(template).into_response()
}
