use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use dropspot_core::integration::integration::{
    Integration as ApiIntegration, UpsertIntegrationPayload,
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

impl From<Integration> for ApiIntegration {
    fn from(integration: Integration) -> Self {
        Self {
            slug: integration.slug.into(),
            is_active: integration.is_active,
            data: integration.data.0,
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

    if let Err(_e) = organisation {
        let api_error = ApiError::new(
            "Failed to load organisation".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        return api_error.into_response();
    };

    let organisation = organisation.unwrap();

    let Ok(storage_type) = StorageType::try_from(slug) else {
        let api_error = ApiError::new("Bad request".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    };

    let result = get_integration_by_slug(&pool, &organisation.id, &storage_type).await;

    if result.is_err() {
        let api_error = ApiError::new("Integration not found".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    }

    let integration = ApiIntegration::from(result.unwrap());
    Json(integration).into_response()
}

pub async fn handle_upsert_integration(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
    Json(payload): Json<UpsertIntegrationPayload>,
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
    let result = upsert_integration(&pool, &organisation.id, &storage_type, &payload.data).await;

    if result.is_err() {
        let api_error = ApiError::new("Integration not found".to_owned(), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    }

    let integration = ApiIntegration::from(result.unwrap());
    Json(integration).into_response()
}
