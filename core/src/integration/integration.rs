use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;
use crate::error::{ApiError, Error};
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

#[wasm_bindgen(js_name = getIntegrations)]
pub async fn get_integrations(auth: Authentication) -> Result<Vec<Integration>, Error> {
    let mut headers = get_auth_headers(Some(&auth));
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let response = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/integrations"))
        .headers(headers)
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
        .json::<Vec<Integration>>()
        .map_err(|_e| Error::JsonError)
        .await?;

    Ok(result)
}

#[wasm_bindgen(js_name = upsertIntegration)]
pub async fn upsert_integration(
    payload: UpsertIntegrationPayload,
    auth: Authentication,
    slug: String,
) -> Result<Integration, Error> {
    let mut headers = get_auth_headers(Some(&auth));
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let response = reqwest::Client::new()
        .patch(format!("{ENDPOINT}/api/integrations/{slug}/upsert"))
        .headers(headers)
        .json(&payload)
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
        .json::<Integration>()
        .map_err(|_e| Error::JsonError)
        .await?;

    Ok(result)
}
