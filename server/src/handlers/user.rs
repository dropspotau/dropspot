use axum::{
    extract::{Json, State},
    http::StatusCode,
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

use crate::{
    auth::password::{hash_password, verify_password},
    db::{
        User, create_user, get_default_organisation, get_organisation_for_user, get_user_by_email,
        get_user_by_id, get_user_password, get_users,
    },
    state::AppState,
    types::ApiError,
};

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

    let Ok(organisation) = get_default_organisation(pool).await else {
        tracing::error!("Could not retrieve default organisation for user creation");
        return ApiError::new(
            "Sorry, there was an error creating your user. Please try again.".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };
    let Ok(existing_users) = get_users(pool, &organisation.id).await else {
        tracing::error!("Organisation users not found: {}", organisation.id);
        return ApiError::new(
            "Sorry, there was an error creating your user. Please try again.".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };
    let is_first_user = existing_users.is_empty();

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
        tracing::error!("Could not hash password");
        return ApiError::new(
            "Sorry, there was an error creating your user. Please try again".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };

    let engine = GeneralPurpose::new(&STANDARD, NO_PAD);
    let password_base64 = engine.encode(password_hash);

    let first_name = payload.first_name.unwrap_or("".to_owned());
    let last_name = payload.last_name.unwrap_or("".to_owned());

    let Ok(user) = create_user(
        pool,
        &payload.email,
        &first_name,
        &last_name,
        &organisation.id,
        is_first_user,
        &password_base64,
    )
    .await
    else {
        tracing::error!("Could not create user");
        return ApiError::new(
            "Sorry, there was an error creating your user. Please try again".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };

    let organisation = get_organisation_for_user(pool, &user.id).await;

    if let Err(e) = organisation {
        tracing::error!("Could not get organisation for new user {}: {e}", user.id);
        return ApiError::new(
            "Could not create user".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    }

    // Generate tokens
    let Ok(tokens) = token_service.generate_token_pair(user.id.clone(), user.email.clone()) else {
        tracing::error!("Could not generate access token");
        return ApiError::new(
            "Sorry, there was an error creating your user. Please try again".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };

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
        tracing::error!("Could not decode refresh token: {e:?}");
        return ApiError::new(
            "Sorry, there was an error signing back in. Please try again.".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };

    let claims = claims.unwrap();
    let user_id = &claims.sub;

    let Ok(user) = get_user_by_id(pool, user_id).await else {
        return ApiError::new(
            "Token refresh user not found".to_owned(),
            StatusCode::NOT_FOUND,
        )
        .into_response();
    };

    let Ok(tokens) = token_service.generate_token_pair(user.id.clone(), user.email.clone()) else {
        tracing::error!("Could not generate access token");
        return ApiError::new(
            "Sorry, there was an error signing back in. Please try again.".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
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
    let default_error = ApiError::new(
        "Invalid email or password".to_owned(),
        StatusCode::FORBIDDEN,
    );
    let Ok(user) = get_user_by_email(pool, &payload.email).await else {
        return default_error.into_response();
    };
    let password_base64 = get_user_password(pool, &user.id).await;

    if let Err(e) = password_base64 {
        tracing::error!("Could not get user password: {}: {e}", user.id);
        return default_error.into_response();
    };

    let engine = GeneralPurpose::new(&STANDARD, NO_PAD);
    let Ok(password) = engine.decode(password_base64.unwrap()) else {
        tracing::error!("Could not decode password");
        return default_error.into_response();
    };

    let Ok(password) = str::from_utf8(&password) else {
        tracing::error!("Could not convert password from UTF-8");
        return default_error.into_response();
    };

    let matches = match verify_password(&payload.password, password) {
        Ok(matches) => matches,
        Err(_) => false,
    };

    if !matches {
        // Invalid password, don't bother logging
        return default_error.into_response();
    }

    // Generate tokens
    let Ok(tokens) = state
        .get_token_service()
        .generate_token_pair(user.id, user.email.clone())
    else {
        tracing::error!("Could not generate tokens for user {}", user.id);
        return ApiError::new(
            "Sorry, there was an error signing in. Please try again.".to_owned(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
        .into_response();
    };

    Json(LoginResult {
        user: ApiUser::from(user),
        tokens,
    })
    .into_response()
}
