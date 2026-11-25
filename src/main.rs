pub mod store;
pub mod shortened_response;
pub mod router;
pub mod shortened_request;

use axum::{routing::get, routing::post, Router};
use crate::router::AppRouter;

#[tokio::main]
async fn main() {
    let store: store::Store = Default::default();
    let app_router = AppRouter {};
    let app = Router::new()
        .route("/health", get(AppRouter::health))
        .route("/shortened", post(AppRouter::shortened))
        .route("/{code}", get(AppRouter::redirect))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on :3000");
    axum::serve(listener, app).await.unwrap();
}
