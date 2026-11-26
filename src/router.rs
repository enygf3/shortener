use std::collections::HashMap;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::Redirect;
use crate::shortened_request::ShortenedRequest;
use crate::shortened_response::ShortenedResponse;
use crate::store;

pub(crate) struct AppRouter { }
impl AppRouter {
    const BASE_URL: &str = "localhost:3000";
    pub(crate) async fn health() -> String {
        String::from("ok")
    }

    pub(crate) async fn shortened(State(store): State<store::Store>, Json(request): Json<ShortenedRequest>) -> ShortenedResponse {
        let url = request.url;
        let host = Self::BASE_URL;
        let result = store.put_url(&url).await;
        let code = result.unwrap();
        let short_url = format!("https://{host}/{code}");
        ShortenedResponse::new(Option::from(HashMap::from([
            (String::from("url"), short_url)
        ])), None)
    }

    pub(crate) async fn redirect(State(store): State<store::Store>, Path(code): Path<String>) -> Result<Redirect, StatusCode> {
        let url = store.get_url(&code).await.unwrap();
        Ok(Redirect::to(&url.unwrap()))
    }
}