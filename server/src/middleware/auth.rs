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
        // Try and get the access token from the Authorization header
        let access_token = if let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        {
            // Standard request, where the access token is in the Authorization header
            bearer.token().to_string()
        } else if let Some(cookies) = parts.headers.typed_get::<Cookie>() {
            // Check cookies in the case of a browser request as a fallback. This lets authorisation
            // possibly be known on the first request
            let cookie_access_token = cookies.get("access_token").map(|t| t.to_string());

            if cookie_access_token.is_none() {
                // There's no access token in the cookies, so the user can't be authenticated
                let api_error = ApiError {
                    message: "Unauthorised".to_string(),
                    status: StatusCode::UNAUTHORIZED,
                };

                return Err(api_error);
            }

            cookie_access_token.unwrap()
        } else {
            // There's still no access token, so the user can't be authenticated
            let api_error = ApiError {
                message: "Unauthorised".to_string(),
                status: StatusCode::UNAUTHORIZED,
            };

            return Err(api_error);
        };

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
