use axum::{
    Json,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub message: String,

    #[serde(skip_serializing, with = "http_serde::status_code")]
    pub(crate) status: StatusCode,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status;
        (status, Json(self)).into_response()
    }
}
