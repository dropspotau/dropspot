use std::collections::HashMap;

use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use dropspot_core::{
    integration::integration::{Integration as ApiIntegration, UpsertIntegrationPayload},
    storage::StorageType as ApiStorageType,
};
use reqwest::StatusCode;

use crate::{
    db::{
        Integration, User, get_integration_by_slug, get_organisation_for_user, upsert_integration,
    },
    state::AppState,
    storage::StorageType,
    types::ApiError,
};

#[derive(Debug, thiserror::Error)]
pub enum IntegrationError {
    #[error("Encryption error: {0}")]
    RetrievalError(sqlx::Error),

    #[error("Invalid payload")]
    PayloadError,
}

impl Into<ApiError> for IntegrationError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<Integration> for ApiIntegration {
    fn from(integration: Integration) -> Self {
        Self {
            slug: integration.slug.into(),
            is_active: integration.is_active,
            data: HashMap::new(),
        }
    }
}

pub async fn handle_get_integration_by_slug(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let pool = state.get_pool();
    let organisation = get_organisation_for_user(pool, &user.id).await;

    if let Err(e) = organisation {
        let error: ApiError = IntegrationError::RetrievalError(e).into();
        return error.into_response();
    };

    let organisation = organisation.unwrap();

    let Ok(storage_type) = StorageType::try_from(slug) else {
        let error: ApiError = IntegrationError::PayloadError.into();
        return error.into_response();
    };

    let result = get_integration_by_slug(&pool, &organisation.id, &storage_type).await;

    if let Err(e) = result {
        let error: ApiError = IntegrationError::RetrievalError(e).into();
        return error.into_response();
    }

    let integration = ApiIntegration::from(result.unwrap());
    Json(integration).into_response()
}

pub async fn handle_upsert_local_integration(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
    Json(payload): Json<UpsertIntegrationPayload>,
) -> impl IntoResponse {
    let pool = state.get_pool();
    let organisation = get_organisation_for_user(pool, &user.id).await;

    if let Err(e) = organisation {
        let error: ApiError = IntegrationError::RetrievalError(e).into();
        return error.into_response();
    };

    let Ok(storage_type) = StorageType::try_from(slug) else {
        let error: ApiError = IntegrationError::PayloadError.into();
        return error.into_response();
    };

    let organisation = organisation.unwrap();
    let result = upsert_integration(&pool, &organisation.id, &storage_type).await;

    if let Err(e) = result {
        let error: ApiError = IntegrationError::RetrievalError(e).into();
        return error.into_response();
    }

    let integration = ApiIntegration::from(result.unwrap());
    Json(integration).into_response()
}
