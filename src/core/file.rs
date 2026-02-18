use crate::server::handlers::ApiFile;

use super::constants::ENDPOINT;

pub async fn list_files() -> Result<Vec<ApiFile>, reqwest::Error> {
    let files = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file"))
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<ApiFile>>()
        .await?;

    Ok(files)
}
