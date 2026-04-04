use axum::{Json, extract::State, response::IntoResponse};
use dropspot_core::integration::local::{
    LocalIntegration as ApiLocalIntegration, UpsertLocalIntegrationPayload,
};
use reqwest::StatusCode;

use crate::{
    db::{LocalIntegration, User, get_organisation_for_user, upsert_local_integration},
    state::AppState,
    types::ApiError,
};

#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Encryption error: {0}")]
    RetrievalError(sqlx::Error),
}

impl Into<ApiError> for IntegrationError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<LocalIntegration> for ApiLocalIntegration {
    fn from(integration: LocalIntegration) -> Self {
        Self {
            slug: "local".to_string(),
            upload_path: integration.upload_path,
            is_active: integration.is_active,
        }
    }
}

pub async fn handle_upsert_local_integration(
    State(state): State<AppState>,
    user: User,
    Json(payload): Json<UpsertLocalIntegrationPayload>,
) -> impl IntoResponse {
    let pool = state.get_pool();
    let organisation = get_organisation_for_user(pool, &user.id).await;

    if let Err(e) = organisation {
        let error: ApiError = IntegrationError::RetrievalError(e).into();
        return error.into_response();
    };

    let organisation = organisation.unwrap();
    let result = upsert_local_integration(&pool, &organisation.id, &payload.upload_path).await;

    if let Err(e) = result {
        let error: ApiError = IntegrationError::RetrievalError(e).into();
        return error.into_response();
    }

    let integration = ApiLocalIntegration::from(result.unwrap());

    Json(integration).into_response()
}
