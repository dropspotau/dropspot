use futures_util::TryFutureExt;
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct UpsertGcsIntegrationPayload {
    pub bucket_name: String,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct GcsIntegration {
    pub bucket_name: String,
}

pub async fn upsert_gcs_integration(
    payload: UpsertGcsIntegrationPayload,
    auth: Authentication,
) -> Result<GcsIntegration, reqwest::Error> {
    let mut headers = get_auth_headers(Some(&auth));
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let result = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/integrations/gcs/upsert"))
        .headers(headers)
        .json(&payload)
        .send()
        .await?
        .json::<GcsIntegration>()
        .await?;

    Ok(result)
}

#[wasm_bindgen]
pub async fn upsert_gcs_integration_js(
    payload: UpsertGcsIntegrationPayload,
    auth: Authentication,
) -> Result<GcsIntegration, JsError> {
    upsert_gcs_integration(payload, auth)
        .map_err(|e| JsError::new(&format!("{e}")))
        .await
}
