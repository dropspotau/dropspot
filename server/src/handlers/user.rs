use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use base64::{
    Engine,
    alphabet::STANDARD,
    engine::{GeneralPurpose, general_purpose::NO_PAD},
};
use dropspot_core::{
    auth::validate_password,
    user::{AccessTokenRequest, CreateUserPayload, LoginPayload, LoginResult, User as ApiUser},
};
use reqwest::StatusCode;
use thiserror::Error;

use crate::{
    auth::password::{hash_password, verify_password},
    db::{
        User, create_user, get_organisation_for_user, get_user_by_email, get_user_by_id,
        get_user_password,
    },
    state::AppState,
    types::ApiError,
};

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Could not create user")]
    CreateUserError(sqlx::Error),

    #[error("Could not create user")]
    CreateUserPasswordError,

    #[error("Could not find user")]
    UserLookupError(sqlx::Error),

    #[error("User not found")]
    UserNotFound,

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
    State(state): State<AppState>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let pool = state.get_pool();

    if let Ok(_existing) = get_user_by_email(pool, &payload.email).await {
        let api_error = ApiError::new(
            "A user with that email already exists".to_owned(),
            StatusCode::CONFLICT,
        );
        return api_error.into_response();
    }

    let password_validation = validate_password(&payload.password);

    if !password_validation.ok {
        let error_messages = password_validation
            .errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        let api_error = ApiError::new(error_messages.join("\n"), StatusCode::BAD_REQUEST);
        return api_error.into_response();
    }

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

    let organisation = get_organisation_for_user(pool, &user.id).await;

    if let Err(e) = organisation {
        let api_error: ApiError = LoginError::CreateUserError(e).into();
        return api_error.into_response();
    }

    // Generate tokens
    let tokens = state
        .get_token_service()
        .generate_token_pair(user.id, user.email.clone())
        .unwrap();

    Json(LoginResult {
        user: ApiUser::from(user),
        tokens,
    })
    .into_response()
}

pub async fn handle_refresh_tokens(
    State(state): State<AppState>,
    Json(payload): Json<AccessTokenRequest>,
) -> impl IntoResponse {
    let pool = state.get_pool();
    let token_service = state.get_token_service();
    let claims = token_service.validate_refresh_token(&payload.refresh_token);

    if let Err(e) = claims {
        eprintln!("Could not decode refresh token: {e:?}");
        let api_error: ApiError = LoginError::UserNotFound.into();
        return api_error.into_response();
    };

    let claims = claims.unwrap();
    let user_id = &claims.sub;

    let Ok(user) = get_user_by_id(pool, user_id).await else {
        let api_error: ApiError = LoginError::UserNotFound.into();
        return api_error.into_response();
    };

    let Ok(tokens) = token_service.generate_token_pair(user.id.clone(), user.email.clone()) else {
        eprintln!("Could not generate access token");
        let api_error: ApiError = LoginError::UserNotFound.into();
        return api_error.into_response();
    };

    Json(LoginResult {
        user: ApiUser::from(user),
        tokens,
    })
    .into_response()
}

pub async fn handle_login(
    State(state): State<AppState>,
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

    // Generate tokens
    let tokens = state
        .get_token_service()
        .generate_token_pair(user.id, user.email.clone())
        .unwrap();

    Json(LoginResult {
        user: ApiUser::from(user),
        tokens,
    })
    .into_response()
}
