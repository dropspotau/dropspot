use bytes::Bytes;
use futures_util::Stream;
use reqwest::Error;
use uuid::Uuid;

use crate::server::handlers::ApiDownload;

use super::constants::ENDPOINT;

pub async fn download(
    file_id: Uuid,
) -> Result<impl Stream<Item = Result<Bytes, Error>> + use<>, reqwest::Error> {
    // Request a download URL
    let download = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/file/{file_id}/download"))
        .send()
        .await?
        .error_for_status()?
        .json::<ApiDownload>()
        .await?;

    // Actually download the file
    let download_id = download.id;
    let stream = reqwest::Client::new()
        .get(format!("{ENDPOINT}/api/download/{download_id}/download"))
        .send()
        .await?
        .bytes_stream();

    Ok(stream)
}
