// src/middleware/auth.rs
// JWT authentication extractor

use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, Cookie, HeaderMapExt, authorization::Bearer},
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
        let mut access_token: Option<String> = None;

        // Try and get the access token from the Authorization header
        if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            access_token = Some(bearer.token().to_string());
        }

        // Check cookies in the case of a browser request as a fallback. This lets authorisation
        // possibly be known on the first request
        if access_token.is_none() {
            let cookies = parts.headers.typed_get::<Cookie>();

            if let Some(cookies) = cookies {
                access_token = cookies.get("access_token").map(|t| t.to_string());
            }
        }

        if access_token.is_none() {
            // There's still no access token, so the user can't be authenticated
            let api_error = ApiError {
                message: "Unauthorised".to_string(),
                status: StatusCode::UNAUTHORIZED,
            };

            return Err(api_error);
        }

        let access_token = access_token.unwrap();

        // Validate token
        let claims = state
            .get_token_service()
            .validate_access_token(&access_token)
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
