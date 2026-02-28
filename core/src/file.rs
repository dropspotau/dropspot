use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use tsify::Tsify;

use crate::constants::ENDPOINT;

#[derive(Serialize, Deserialize, Tsify, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub size: i64,
}

pub async fn list_files() -> Result<Vec<File>, reqwest::Error> {
    let files = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file"))
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<File>>()
        .await?;

    Ok(files)
}

pub async fn get_file(id: &Uuid) -> Result<File, reqwest::Error> {
    let file = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file/{id}"))
        .send()
        .await?
        .error_for_status()?
        .json::<File>()
        .await?;

    Ok(file)
}
