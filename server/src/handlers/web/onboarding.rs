use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};

use crate::{
    db::{User, record_onboarding_completion},
    handlers::web::template::HtmlTemplate,
    state::AppState,
};

#[derive(Template)]
#[template(path = "onboarding_record.html")]
struct RecordOnboardingTemplate {
    success: bool,
}

#[derive(Template)]
#[template(path = "settings_unauthed.html")]
struct SettingsUnAuthedTemplate {}

pub async fn handle_record_onboarding(State(state): State<AppState>, user: User) -> Response {
    let pool = state.get_pool();
    let onboarding = record_onboarding_completion(pool, &user.id).await;
    let success = onboarding.is_ok();

    if let Err(e) = onboarding {
        tracing::error!("Onboarding error: {e} user_id={}", &user.id);
    }

    let template = RecordOnboardingTemplate { success };
    HtmlTemplate(template).into_response()
}
