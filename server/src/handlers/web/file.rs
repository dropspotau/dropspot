use askama::Template;
use axum::{
    extract::{Path, State},
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
    db::{File, delete_files, get_file_by_id, get_files},
    state::AppState,
};

use super::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "files.html")]
struct FilesTemplate {
    files: Vec<File>,
    is_empty: bool,
}

pub async fn handle_files(State(state): State<AppState>) -> impl IntoResponse {
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

#[derive(Template)]
#[template(path = "delete_file.html")]
struct DeleteFileTemplate {
    deleted: bool,
}

pub async fn handle_delete_file(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &id).await else {
        let template = DeleteFileTemplate { deleted: false };
        let response = HtmlTemplate(template).into_response();

        return response;
    };

    if let Err(_e) = delete_files(pool, &[file.id]).await {
        let template = DeleteFileTemplate { deleted: false };
        let response = HtmlTemplate(template).into_response();

        return response;
    }

    if file.delete_file().is_err() {
        eprintln!("Failed to delete file: {}", file.id);
    }

    let template = DeleteFileTemplate { deleted: true };
    let mut response = HtmlTemplate(template).into_response();

    let headers = response.headers_mut();
    headers.insert("HX-Trigger", HeaderValue::from_str("file-delete").unwrap());

    response
}
