// src/middleware/auth.rs
// JWT authentication extractor

use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{db::User, state::AppState};
use crate::{db::get_user_by_id, types::ApiError};

/// Authenticated user information extracted from JWT

impl FromRequestParts<AppState> for User {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        else {
            let api_error = ApiError {
                message: "Unauthorised".to_string(),
                status: StatusCode::UNAUTHORIZED,
            };
            return Err(api_error);
        };

        // Validate token
        let claims = state
            .get_token_service()
            .validate_access_token(bearer.token())
            .map_err(|_| ApiError {
                message: "Unauthorised".to_string(),
                status: StatusCode::UNAUTHORIZED,
            })?;

        let user_id = claims.sub;

        let Ok(user) = get_user_by_id(state.get_pool(), &user_id).await else {
            let api_error = ApiError {
                message: "Unauthorised".to_string(),
                status: StatusCode::UNAUTHORIZED,
            };
            return Err(api_error);
        };

        Ok(user)
    }
}

impl FromRequestParts<AppState> for Option<User> {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match User::from_request_parts(parts, state).await {
            Ok(user) => Ok(Some(user)),
            Err(_) => Ok(None),
        }
    }
}
