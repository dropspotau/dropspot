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
    // TODO(alec): Make this return an object with a download ID and URL
    let download = reqwest::Client::new()
        .get(format!("{ENDPOINT}/download/{file_id}"))
        .send()
        .await?
        .json::<ApiDownload>()
        .await?;

    // Actually download the file
    let download_id = download.id;
    let stream = reqwest::Client::new()
        .get(format!("{ENDPOINT}/download/{download_id}"))
        .send()
        .await?
        .bytes_stream();

    // TODO(alec): Return something from the standard library

    Ok(stream)
}
