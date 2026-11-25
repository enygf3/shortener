use std::collections::HashMap;
use axum::Json;
use axum::response::IntoResponse;

#[derive(serde::Serialize, Debug)]
pub(crate) struct ShortenedResponse {
    message: Option<String>,
    data: Option<HashMap<String, String>>,
}

impl ShortenedResponse {
    pub fn new(data: Option<HashMap<String, String>>, message: Option<String>) -> Self {
        Self { message, data }
    }
}

impl IntoResponse for ShortenedResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}