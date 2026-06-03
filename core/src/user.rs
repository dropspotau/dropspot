use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::{JsError, prelude::wasm_bindgen};

use crate::constants::ENDPOINT;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Encryption error: {0}")]
    LoginError(reqwest::Error),

    #[error("Upload error: {0}")]
    CreateError(reqwest::Error),
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

/// Token pair returned to client
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LoginResult {
    pub user: User,
    pub tokens: TokenPair,
}

pub async fn create_user(
    email: String,
    password: String,
    first_name: String,
    last_name: String,
) -> Result<LoginResult, UserError> {
    let result = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/user/create"))
        .header("Content-Type", "application/json")
        .json(&CreateUserPayload {
            first_name,
            last_name,
            email,
            password,
        })
        .send()
        .await
        .map_err(UserError::CreateError)?
        .json::<LoginResult>()
        .await
        .map_err(UserError::CreateError)?;

    Ok(result)
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn login(email: String, password: String) -> Result<LoginResult, UserError> {
    let result = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/user/login"))
        .header("Content-Type", "application/json")
        .json(&LoginPayload { email, password })
        .send()
        .await
        .map_err(UserError::LoginError)?
        .json::<LoginResult>()
        .await
        .map_err(UserError::LoginError)?;

    Ok(result)
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AccessTokenRequest {
    pub refresh_token: String,
}

pub async fn refresh_tokens(refresh_token: String) -> Result<LoginResult, UserError> {
    let result = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/user/refresh"))
        .header("Content-Type", "application/json")
        .json(&AccessTokenRequest { refresh_token })
        .send()
        .await
        .map_err(UserError::LoginError)?
        .json::<LoginResult>()
        .await
        .map_err(UserError::LoginError)?;

    Ok(result)
}

#[wasm_bindgen(js_name = createUser)]
pub async fn create_user_js(
    email: String,
    first_name: String,
    last_name: String,
    password: String,
) -> Result<LoginResult, JsError> {
    let result = create_user(email, password, first_name, last_name).await;

    if let Err(e) = result {
        return Err(JsError::new(&format!("{e:?}")));
    };

    let user = result.unwrap();
    Ok(user)
}

#[wasm_bindgen(js_name = login)]
pub async fn login_js(email: String, password: String) -> Result<LoginResult, JsError> {
    let result = login(email, password).await;

    if let Err(e) = result {
        return Err(JsError::new(&format!("{e:?}")));
    };

    let user = result.unwrap();
    Ok(user)
}

#[wasm_bindgen(js_name = refreshTokens)]
pub async fn refresh_tokens_js(refresh_token: String) -> Result<LoginResult, JsError> {
    let result = refresh_tokens(refresh_token).await;

    if let Err(e) = result {
        return Err(JsError::new(&format!("{e:?}")));
    };

    let user = result.unwrap();
    Ok(user)
}
