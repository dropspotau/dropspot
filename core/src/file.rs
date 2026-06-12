use base64::{
    Engine,
    alphabet::URL_SAFE,
    engine::{GeneralPurpose, general_purpose::NO_PAD},
};
use chrono::{DateTime, Utc};
use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::{
    auth::{Authentication, get_auth_headers},
    constants::ENDPOINT,
    encryption::Encryption,
    error::{ApiError, Error},
};

#[derive(Serialize, Deserialize, Tsify, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub size: i64,
    pub remaining_downloads: i32,
    pub is_expired: bool,
}

pub async fn list_files(auth: Option<&Authentication>) -> Result<Vec<File>, reqwest::Error> {
    let mut headers = get_auth_headers(auth);
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let files = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file"))
        .headers(headers)
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<File>>()
        .await?;

    Ok(files)
}

pub async fn get_file(id: &Uuid, auth: Option<&Authentication>) -> Result<File, reqwest::Error> {
    let mut headers = get_auth_headers(auth);
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let file = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file/{id}"))
        .headers(headers)
        .send()
        .await?
        .error_for_status()?
        .json::<File>()
        .await?;

    Ok(file)
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct UpdateFilePayload {
    pub expires_at: Option<DateTime<Utc>>,
    pub max_downloads: Option<i32>,
}

pub async fn update_file(
    id: &Uuid,
    auth: &Authentication,
    payload: &UpdateFilePayload,
) -> Result<File, Error> {
    let mut headers = get_auth_headers(Some(auth));
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let response = reqwest::Client::new()
        .patch(format!("{ENDPOINT}/api/file/{id}"))
        .headers(headers)
        .json(payload)
        .send()
        .map_err(|_e| Error::NetworkError)
        .await?;

    if !response.status().is_success() {
        return Err(response
            .json::<ApiError>()
            .await
            .map(Error::ApiError)
            .map_err(|_e| Error::JsonError)?);
    }

    response.json::<File>().map_err(|_e| Error::JsonError).await
}

pub async fn delete_file(id: &Uuid, auth: &Authentication) -> Result<(), reqwest::Error> {
    let mut headers = get_auth_headers(Some(auth));
    headers.insert("Content-Type", "application/json".parse().unwrap());

    reqwest::Client::new()
        .delete(format!("{ENDPOINT}/api/file/{id}"))
        .headers(headers)
        .send()
        .await?
        .error_for_status()
        .map(|_r| ())
}

pub fn create_link(file_id: &Uuid, encryption: &Encryption) -> String {
    let engine = GeneralPurpose::new(&URL_SAFE, NO_PAD);
    let key_base64 = engine.encode(&encryption.key);
    let nonce_base64 = engine.encode(&encryption.nonce);

    format!("{file_id};{key_base64};{nonce_base64}")
}

pub fn parse_link(link: &str) -> Result<(Uuid, Encryption), String> {
    let mut parts = link.split(';');
    let file_id = parts.next().ok_or("Missing file_id")?;
    let key_base64 = parts.next().ok_or("Missing key")?;
    let nonce_base64 = parts.next().ok_or("Missing nonce")?;

    let engine = GeneralPurpose::new(&URL_SAFE, NO_PAD);
    let key = engine.decode(key_base64).map_err(|e| e.to_string())?;
    let nonce = engine.decode(nonce_base64).map_err(|e| e.to_string())?;

    let file_id = Uuid::parse_str(file_id).map_err(|e| e.to_string())?;

    Ok((file_id, Encryption { key, nonce }))
}

// A link to a fail. Named struct because wasm_bindgen can't return tuple structs
#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Link {
    file_id: String,
    encryption: Encryption,
}

#[wasm_bindgen(js_name = createLink)]
pub fn create_link_js(file_id: String, encryption: Encryption) -> Result<String, JsError> {
    let file_id = Uuid::parse_str(&file_id).map_err(|e| JsError::new(&format!("{e}")))?;
    Ok(create_link(&file_id, &encryption))
}

#[wasm_bindgen(js_name = parseLink)]
pub fn parse_link_js(link: &str) -> Result<Link, JsError> {
    let (file_id, encryption) = parse_link(link).map_err(|err| JsError::new(&format!("{err}")))?;

    Ok(Link {
        file_id: file_id.to_string(),
        encryption: encryption,
    })
}

#[wasm_bindgen(js_name = getFile)]
pub async fn get_file_js(file_id: String, auth: Option<Authentication>) -> Result<File, JsError> {
    let file_id = Uuid::parse_str(&file_id).map_err(|e| JsError::new(&format!("{e}")))?;
    get_file(&file_id, auth.as_ref())
        .await
        .map_err(|e| JsError::new(&format!("{e}")))
}

#[wasm_bindgen(js_name = updateFile)]
pub async fn update_file_js(
    id: String,
    auth: Authentication,
    payload: UpdateFilePayload,
) -> Result<File, Error> {
    let file_id = Uuid::parse_str(&id).map_err(|_e| Error::WasmConversionError)?;
    update_file(&file_id, &auth, &payload).await
}

#[wasm_bindgen(js_name = deleteFile)]
pub async fn delete_file_js(file_id: String, auth: Authentication) -> Result<(), JsError> {
    let file_id = Uuid::parse_str(&file_id).map_err(|e| JsError::new(&format!("{e}")))?;
    delete_file(&file_id, &auth)
        .await
        .map_err(|e| JsError::new(&format!("{e}")))
}
