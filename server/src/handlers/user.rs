use std::sync::Arc;

use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use base64::{
    Engine,
    alphabet::STANDARD,
    engine::{GeneralPurpose, general_purpose::NO_PAD},
};
use dropspot_core::user::{CreateUserPayload, LoginPayload, User as ApiUser};
use reqwest::StatusCode;
use thiserror::Error;

use crate::{
    auth::hash_password,
    db::{User, create_user},
};
use crate::{auth::verify_password, db::get_user_password, types::ApiError};
use crate::{db::get_user_by_email, state::AppState};

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Could not create user")]
    CreateUserError(sqlx::Error),

    #[error("Could not create user")]
    CreateUserPasswordError,

    #[error("Could not find user")]
    UserLookupError(sqlx::Error),

    #[error("Passwords do not match")]
    PasswordMismatch,
}

impl Into<ApiError> for LoginError {
    fn into(self) -> ApiError {
        ApiError {
            message: self.to_string(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<User> for ApiUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
        }
    }
}

pub async fn handle_create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let pool = state.get_pool();

    let Ok(password_hash) = hash_password(&payload.password) else {
        let api_error: ApiError = LoginError::CreateUserPasswordError.into();
        return api_error.into_response();
    };

    let engine = GeneralPurpose::new(&STANDARD, NO_PAD);
    let password_base64 = engine.encode(password_hash);

    let user = create_user(
        pool,
        &payload.first_name,
        &payload.last_name,
        &payload.email,
        &password_base64,
    )
    .await
    .unwrap();

    Json(ApiUser::from(user)).into_response()
}

pub async fn handle_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let pool = state.get_pool();
    let user = get_user_by_email(pool, &payload.email).await;

    if let Err(e) = user {
        let api_error: ApiError = LoginError::UserLookupError(e).into();
        return api_error.into_response();
    };

    let user = user.unwrap();
    let password_base64 = get_user_password(pool, &user.id).await;

    println!("password_base64: {password_base64:?}");

    if let Err(e) = password_base64 {
        let api_error: ApiError = LoginError::UserLookupError(e).into();
        return api_error.into_response();
    };

    let engine = GeneralPurpose::new(&STANDARD, NO_PAD);
    let Ok(password) = engine.decode(password_base64.unwrap()) else {
        eprintln!("Could not decode");
        let api_error: ApiError = LoginError::UserLookupError(sqlx::Error::RowNotFound).into();
        return api_error.into_response();
    };

    let Ok(password) = str::from_utf8(&password) else {
        eprintln!("Could not decode 2");
        let api_error: ApiError = LoginError::UserLookupError(sqlx::Error::RowNotFound).into();
        return api_error.into_response();
    };

    let matches = match verify_password(&payload.password, password) {
        Ok(matches) => matches,
        Err(_) => false,
    };

    if !matches {
        let api_error: ApiError = LoginError::PasswordMismatch.into();
        return api_error.into_response();
    }

    Json(ApiUser::from(user)).into_response()
}
