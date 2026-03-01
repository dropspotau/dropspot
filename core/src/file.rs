use base64::{
    Engine,
    alphabet::URL_SAFE,
    engine::{GeneralPurpose, general_purpose::NO_PAD},
};
use serde::{Deserialize, Serialize};
use tsify::Tsify;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

use crate::{constants::ENDPOINT, encryption::Encryption};

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

#[wasm_bindgen]
pub fn create_link_js(file_id: String, encryption: Encryption) -> Result<String, JsError> {
    let file_id = Uuid::parse_str(&file_id).map_err(|e| JsError::new(&format!("{e}")))?;
    Ok(create_link(&file_id, &encryption))
}

#[wasm_bindgen]
pub fn parse_link_js(link: &str) -> Result<Link, JsError> {
    let (file_id, encryption) = parse_link(link).map_err(|err| JsError::new(&format!("{err}")))?;

    Ok(Link {
        file_id: file_id.to_string(),
        encryption: encryption,
    })
}
