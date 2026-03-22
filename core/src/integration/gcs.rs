use serde::{Deserialize, Serialize};

use crate::auth::{Authentication, get_auth_headers};
use crate::constants::ENDPOINT;

#[derive(Serialize, Deserialize)]
pub struct UpsertGcsIntegrationPayload {
    pub bucket_name: String,
}

#[derive(Serialize, Deserialize)]
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
