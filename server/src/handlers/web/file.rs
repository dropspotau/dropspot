use std::sync::Arc;

use askama::Template;
use axum::{extract::State, response::IntoResponse};

use crate::{
    db::{File, get_files},
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "files.html")]
struct FilesTemplate {
    files: Vec<File>,
    is_empty: bool,
}

pub async fn handle_files(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let pool = state.get_pool();
    let files = match get_files(pool).await {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error getting files: {e:?}");
            vec![]
        }
    };
    let is_empty = files.is_empty();

    let template = FilesTemplate { files, is_empty };
    HtmlTemplate(template)
}
