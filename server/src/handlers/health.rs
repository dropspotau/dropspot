use axum::{
    extract::Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
}

pub async fn handle_health() -> Response {
    Json(HealthResponse { ok: true }).into_response()
}
