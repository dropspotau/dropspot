use std::collections::HashMap;

use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
    db::{Download, File, User, get_downloads_for_file, get_files},
    permissions::file::can_see_file,
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "files.html")]
struct FilesTemplate {
    files: Vec<File>,
    downloads_by_file: HashMap<Uuid, Vec<Download>>,
    is_empty: bool,
}

#[derive(Template)]
#[template(path = "files_unauthed.html")]
struct FilesUnAuthedTemplate {}

pub async fn handle_files(State(state): State<AppState>, user: Option<User>) -> Response {
    if user.is_none() {
        let template = FilesUnAuthedTemplate {};
        return HtmlTemplate(template).into_response();
    }

    let user = user.unwrap();
    let pool = state.get_pool();
    let files = match get_files(pool).await {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error getting files: {e:?}");
            vec![]
        }
    }
    .into_iter()
    .filter(|file| !file.is_expired() && can_see_file(file, Some(&user)))
    .collect::<Vec<File>>();
    let is_empty = files.is_empty();

    // Mapping of file IDs to their downloads
    let mut downloads_by_file = HashMap::<Uuid, Vec<Download>>::new();

    for file in &files {
        // TODO(alec): This is a nasty N+1 query. It would be good to make the database do the
        // download retrieval all at once
        let downloads = match get_downloads_for_file(pool, &file.id).await {
            Ok(downloads) => downloads,
            Err(e) => {
                tracing::error!("Error retrieving downloads for file {}: {e}", file.id);
                vec![]
            }
        };
        downloads_by_file.insert(file.id, downloads);
    }

    let template = FilesTemplate {
        files,
        downloads_by_file,
        is_empty,
    };
    HtmlTemplate(template).into_response()
}
