use axum::{
    Form, Json,
    extract::{Path, State},
    response::IntoResponse,
};
use dropspot_core::integration::integration::{
    Integration as ApiIntegration, UpsertIntegrationPayload,
};
use reqwest::StatusCode;

use crate::{
    db::{
        Integration, User, get_integration_by_slug, get_integrations, get_organisation_for_user,
        upsert_integration,
    },
    handlers::web::template::HtmlTemplate,
    state::AppState,
    storage::StorageType,
    types::ApiError,
};

use super::settings::UpdateSettingsTemplate;

// NOTE(alec): This is duplicated from the API handle_upsert_integration view but uses a form
// isntead
pub async fn handle_upsert_integration(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
    Form(payload): Form<UpsertIntegrationPayload>,
) -> impl IntoResponse {
    let pool = state.get_pool();
    let organisation = get_organisation_for_user(pool, &user.id).await;

    if organisation.is_err() {
        let api_error = ApiError::new(
            "Failed to load organisation".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    };

    let Ok(storage_type) = StorageType::try_from(slug) else {
        let api_error = ApiError::new("Bad request".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    };

    let organisation = organisation.unwrap();
    let result = upsert_integration(&pool, &organisation.id, &storage_type, payload.is_active, &payload.data).await;

    if result.is_err() {
        let api_error = ApiError::new("Integration not found".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    }

    let template = UpdateSettingsTemplate { success: true };
    HtmlTemplate(template).into_response()
}
