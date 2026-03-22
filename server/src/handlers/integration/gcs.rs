use axum::{Json, extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::{
    db::{User, get_organisation_for_user, upsert_gcs_integration},
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

#[derive(Deserialize)]
pub struct UpsertGcsIntegrationPayload {
    bucket_name: String,
}

pub async fn handle_upsert_gcs_integration(
    State(state): State<AppState>,
    user: User,
    Json(payload): Json<UpsertGcsIntegrationPayload>,
) -> impl IntoResponse {
    let pool = state.get_pool();
    let organisation = get_organisation_for_user(pool, &user.id).await;

    if let Err(e) = organisation {
        let error: ApiError = IntegrationError::RetrievalError(e).into();
        return error.into_response();
    };

    let organisation = organisation.unwrap();

    let result = upsert_gcs_integration(&pool, &organisation.id, &payload.bucket_name).await;

    if let Err(e) = result {
        let error: ApiError = IntegrationError::RetrievalError(e).into();
        return error.into_response();
    }

    Json(result.unwrap()).into_response()
}
