use crate::server::db::{File, Upload};

use super::constants::ENDPOINT;

// TODO(alec): Add Axum and actually upload a file
pub async fn upload(name: String, contents: Vec<u8>) -> Result<File, reqwest::Error> {
    // Request an upload
    let upload = reqwest::Client::new()
        .post(format!("{ENDPOINT}/upload"))
        .send()
        .await?
        .json::<Upload>()?;

    let size = contents.len();

    let file = reqwest::Client::new()
        .post(format!("{ENDPOINT}/file"))
        .send()
        .await?
        .json::<File>()?;

    Ok(file)
}
