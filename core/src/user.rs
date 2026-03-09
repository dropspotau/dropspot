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

#[derive(Serialize, Deserialize)]
pub struct CreateUserPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    email: String,
    password: String,
    first_name: String,
    last_name: String,
) -> Result<User, UserError> {
    let user = reqwest::Client::new()
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
        .json::<User>()
        .await
        .map_err(UserError::CreateError)?;

    Ok(user)
}

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn login(email: String, password: String) -> Result<User, UserError> {
    let user = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/user/login"))
        .header("Content-Type", "application/json")
        .json(&LoginPayload { email, password })
        .send()
        .await
        .map_err(UserError::LoginError)?
        .json::<User>()
        .await
        .map_err(UserError::LoginError)?;

    Ok(user)
}

#[wasm_bindgen]
pub async fn create_user_js(
    email: String,
    first_name: String,
    last_name: String,
    password: String,
) -> Result<User, JsError> {
    let user = create_user(email, password, first_name, last_name).await;

    if let Err(e) = user {
        return Err(JsError::new(&format!("{e:?}")));
    };

    let user = user.unwrap();
    Ok(user)
}

#[wasm_bindgen]
pub async fn login_js(email: String, password: String) -> Result<User, JsError> {
    let user = login(email, password).await;

    if let Err(e) = user {
        return Err(JsError::new(&format!("{e:?}")));
    };

    let user = user.unwrap();
    Ok(user)
}
