use askama::Template;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
};

use crate::{
    db::{File, User, get_files},
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "files.html")]
struct FilesTemplate {
    files: Vec<File>,
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
    let pool = state.get_pool();
    let files = match get_files(pool).await {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error getting files: {e:?}");
            vec![]
        }
    }
    .into_iter()
    .filter(|file| !file.is_expired())
    .collect::<Vec<File>>();
    let is_empty = files.is_empty();

    let template = FilesTemplate { files, is_empty };
    HtmlTemplate(template).into_response()
}
