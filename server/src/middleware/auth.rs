// src/middleware/auth.rs
// JWT authentication extractor

use std::sync::Arc;

use axum::{
    RequestPartsExt,
    extract::{FromRequestParts, State},
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use uuid::Uuid;

use crate::state::AppState;
use crate::{auth::claims::AccessClaims, types::ApiError};

/// Authenticated user information extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}

impl From<AccessClaims> for AuthUser {
    fn from(claims: AccessClaims) -> Self {
        Self {
            id: claims.sub,
            email: claims.email,
        }
    }
}

impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
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

        Ok(AuthUser::from(claims))
    }
}

/// Optional authentication - doesn't fail if no token provided
pub struct OptionalAuthUser(pub Option<AuthUser>);

impl FromRequestParts<AppState> for OptionalAuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(user) => Ok(OptionalAuthUser(Some(user))),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}
