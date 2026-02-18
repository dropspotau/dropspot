use crate::server::handlers::{ApiFile, CreateFileBody};

use super::constants::ENDPOINT;

pub async fn upload(name: String, contents: Vec<u8>) -> Result<ApiFile, reqwest::Error> {
    let size = contents.len();

    // Request an upload
    let file = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/upload"))
        .header("Content-Type", "application/json")
        .json(&CreateFileBody {
            name,
            size: size as i64,
        })
        .send()
        .await?
        .json::<ApiFile>()
        .await?;

    // Upload the file body
    let file_stream = reqwest::Client::new()
        .post(format!("{ENDPOINT}/api/upload/{}/upload", file.id))
        .header("Content-Type", "application/octet-stream")
        .body(contents)
        .send()
        .await?;

    if let Err(err) = file_stream.error_for_status() {
        return Err(err);
    }

    Ok(file)
}
