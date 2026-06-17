use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use crate::constants::ENDPOINT;
use crate::error::{ApiError, Error};

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Settings {
    pub default_file_expiry_minutes: i32,
    pub default_download_limit: i32,
    pub allow_external_uploads: bool,
    pub allow_external_downloads: bool,
}

#[derive(Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct UpdateSettingsPayload {
    pub file_expiry_minutes: i32,
    pub download_limit: i32,
    pub allow_external_uploads: bool,
    pub allow_external_downloads: bool,
}

impl UpdateSettingsPayload {
    pub fn validate(&self) -> bool {
        self.file_expiry_minutes > 0 && self.download_limit > 0
    }
}

#[wasm_bindgen(js_name = updateSettings)]
pub async fn update_settings(payload: UpdateSettingsPayload) -> Result<Settings, Error> {
    let response = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/settings"))
        .header("Content-Type", "application/json")
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
        .json::<Settings>()
        .await
        .map_err(|_e| Error::JsonError)?;

    Ok(result)
}
