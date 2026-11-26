use std::collections::HashMap;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::Redirect;
use crate::shortened_request::ShortenedRequest;
use crate::shortened_response::ShortenedResponse;
use crate::state::AppState;
use crate::store;

pub(crate) struct AppRouter { }
impl AppRouter {
    pub(crate) async fn health() -> String {
        String::from("ok")
    }

    pub(crate) async fn shortened(State(state): State<AppState>, Json(request): Json<ShortenedRequest>) -> Result<ShortenedResponse, StatusCode> {
        let url = request.url;
        let host = state.base_url;
        let code = state.store
            .put_url(&url)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let short_url = format!("{}/{}", host, code);
        Ok(
            ShortenedResponse::new(Option::from(HashMap::from([
                (String::from("url"), short_url)
            ])), None)
        )
    }

    pub(crate) async fn redirect(State(state): State<AppState>, Path(code): Path<String>) -> Result<Redirect, StatusCode> {
        let url = state.store.get_url(&code).await.unwrap();
        Ok(Redirect::to(&url.unwrap()))
    }
}