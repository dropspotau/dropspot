use crate::server::handlers::{ApiUpload, CreateUploadBody};

use super::constants::ENDPOINT;

pub async fn upload(name: String, contents: Vec<u8>) -> Result<ApiUpload, reqwest::Error> {
    let size = contents.len();

    // Request an upload
    let upload = reqwest::Client::new()
        .post(format!("{ENDPOINT}/upload"))
        .header("Content-Type", "application/json")
        .json(&CreateUploadBody { name, size })
        .send()
        .await?
        .json::<ApiUpload>()
        .await?;

    let file = reqwest::Client::new()
        .post(format!("{ENDPOINT}/upload/{}", upload.id))
        .header("Content-Type", "application/octet-stream")
        .send()
        .await?;

    if let Err(err) = file.error_for_status() {
        return Err(err);
    }

    Ok(upload)
}
