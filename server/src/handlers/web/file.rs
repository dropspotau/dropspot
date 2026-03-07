use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
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

#[derive(Template)]
#[template(path = "delete_file.html")]
struct DeleteFileTemplate {
    deleted: bool,
}

pub async fn handle_delete_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let pool = state.get_pool();

    let Ok(file) = get_file_by_id(pool, &id).await else {
        let template = DeleteFileTemplate { deleted: false };
        return HtmlTemplate(template);
    };

    if let Err(_e) = delete_files(pool, &[file.id]).await {
        let template = DeleteFileTemplate { deleted: false };
        return HtmlTemplate(template);
    }

    let template = DeleteFileTemplate { deleted: true };
    HtmlTemplate(template)
}
