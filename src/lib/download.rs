use std::io::Bytes;

use futures_util::StreamExt;
use reqwest::Url;
use uuid::Uuid;

use crate::db::Download;

use super::constants::ENDPOINT;

pub async fn download(
    download_id: Uuid,
) -> Result<impl Iterator<Item = Bytes> + use<>, reqwest::Error> {
    // Request a download URL
    // TODO(alec): Make this return an object with a download ID and URL
    let download = reqwest::Client::new()
        .get(format!("{ENDPOINT}/download"))
        .send()
        .await?.json::<Download>().await?;

    // Actually download the file
    let download_id = download.id;
    let url = Url::parse_with_params
    let stream = reqwest::Client::new()
        .get(format!("{ENDPOINT}/download/{download_id}"))
        .send()
        .await?.bytes_stream();

    // TODO(alec): Return something from the standard library
    let stream = stream.map(|result| result);

    // Ok(stream)
}
