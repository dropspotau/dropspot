use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct UpsertLocalIntegrationPayload {
    pub upload_path: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct LocalIntegration {
    pub slug: String,
    pub upload_path: String,
    pub is_active: bool,
}

pub async fn upsert_local_integration(
    payload: UpsertLocalIntegrationPayload,
    auth: Authentication,
) -> Result<LocalIntegration, reqwest::Error> {
    let mut headers = get_auth_headers(Some(&auth));
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let result = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/integrations/local/upsert"))
        .headers(headers)
        .json(&payload)
        .send()
        .await?
        .json::<LocalIntegration>()
        .await?;

    Ok(result)
}

#[wasm_bindgen]
pub async fn upsert_local_integration_js(
    payload: UpsertLocalIntegrationPayload,
    auth: Authentication,
) -> Result<LocalIntegration, JsError> {
    upsert_local_integration(payload, auth)
        .map_err(|e| JsError::new(&format!("{e}")))
        .await
}
