use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::ENDPOINT;

#[derive(Serialize, Deserialize, Debug)]
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
