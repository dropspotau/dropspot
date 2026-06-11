use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    constants::ENDPOINT,
    error::{ApiError, Error},
};

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
) -> Result<LoginResult, Error> {
    let response = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/user/create"))
        .header("Content-Type", "application/json")
        .json(&CreateUserPayload {
            email,
            first_name,
            last_name,
            password,
        })
        .send()
        .await
        .map_err(|_e| Error::NetworkError)?;

    if !response.status().is_success() {
        return Err(response
            .json::<ApiError>()
            .await
            .map(Error::ApiError)
            .map_err(|_e| Error::JsonError)
            .unwrap());
    }

    let result = response
        .json::<LoginResult>()
        .map_err(|_e| Error::JsonError)
        .await?;

    Ok(result)
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[wasm_bindgen(js_name = login)]
pub async fn login(email: String, password: String) -> Result<LoginResult, Error> {
    let response = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/user/login"))
        .header("Content-Type", "application/json")
        .json(&LoginPayload { email, password })
        .send()
        .await
        .map_err(|_e| Error::NetworkError)?;

    if !response.status().is_success() {
        return Err(response
            .json::<ApiError>()
            .await
            .map(Error::ApiError)
            .map_err(|_e| Error::JsonError)?);
    }

    let result = response
        .json::<LoginResult>()
        .map_err(|_e| Error::JsonError)
        .await?;

    Ok(result)
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct AccessTokenRequest {
    pub refresh_token: String,
}

#[wasm_bindgen(js_name = refreshTokens)]
pub async fn refresh_tokens(refresh_token: String) -> Result<LoginResult, Error> {
    let response = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/user/refresh"))
        .header("Content-Type", "application/json")
        .json(&AccessTokenRequest { refresh_token })
        .send()
        .await
        .map_err(|_e| Error::NetworkError)?;

    if !response.status().is_success() {
        return Err(response
            .json::<ApiError>()
            .await
            .map(Error::ApiError)
            .map_err(|_e| Error::JsonError)
            .unwrap());
    }

    let result = response
        .json::<LoginResult>()
        .map_err(|_e| Error::JsonError)
        .await?;

    Ok(result)
}
