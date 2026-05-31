use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;
use crate::storage::StorageType;

#[derive(Serialize, Deserialize, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LocalIntegrationData {
    pub folder: String,
}

#[derive(Serialize, Deserialize, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GcsIntegrationData {
    pub bucket_name: String,
}

#[derive(Serialize, Deserialize, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
#[serde(untagged)]
pub enum IntegrationData {
    Local(LocalIntegrationData),
    Gcs(GcsIntegrationData),
}

impl IntegrationData {
    pub fn to_map(&self) -> Vec<(String, String)> {
        match self {
            IntegrationData::Local(data) => {
                vec![("folder".to_string(), data.folder.clone())]
            }
            IntegrationData::Gcs(data) => {
                vec![("bucket_name".to_string(), data.bucket_name.clone())]
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct UpsertIntegrationPayload {
    pub is_active: bool,
    pub data: IntegrationData,
}

#[derive(Serialize, Deserialize, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Integration {
    pub slug: StorageType,
    pub name: String,
    pub is_active: bool,
    pub data: IntegrationData,
}

pub async fn get_integrations(auth: Authentication) -> Result<Vec<Integration>, reqwest::Error> {
    let mut headers = get_auth_headers(Some(&auth));
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let result = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/integrations"))
        .headers(headers)
        .send()
        .await?
        .json::<Vec<Integration>>()
        .await?;

    Ok(result)
}

#[wasm_bindgen]
pub async fn get_integrations_js(auth: Authentication) -> Result<Vec<Integration>, JsError> {
    get_integrations(auth)
        .map_err(|e| JsError::new(&format!("{e}")))
        .await
}

pub async fn upsert_integration(
    payload: UpsertIntegrationPayload,
    auth: Authentication,
    slug: String,
) -> Result<Integration, reqwest::Error> {
    let mut headers = get_auth_headers(Some(&auth));
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let result = reqwest::Client::new()
        .patch(format!("{ENDPOINT}/api/integrations/{slug}/upsert"))
        .headers(headers)
        .json(&payload)
        .send()
        .await?
        .json::<Integration>()
        .await?;

    Ok(result)
}

#[wasm_bindgen]
pub async fn upsert_integration_js(
    payload: UpsertIntegrationPayload,
    auth: Authentication,
    slug: String,
) -> Result<Integration, JsError> {
    upsert_integration(payload, auth, slug)
        .map_err(|e| JsError::new(&format!("{e}")))
        .await
}
