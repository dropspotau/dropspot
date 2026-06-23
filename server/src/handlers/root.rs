use axum::response::{IntoResponse, Redirect, Response};

pub async fn handle_root() -> Response {
    Redirect::permanent("/app").into_response()
}
