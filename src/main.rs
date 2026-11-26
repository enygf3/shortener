pub mod store;
pub mod shortened_response;
pub mod router;
pub mod shortened_request;
pub mod state;

use sqlx::postgres::PgPoolOptions;
use axum::{routing::get, routing::post, Router};
use crate::router::AppRouter;
use crate::state::AppState;
use crate::store::Store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&url).await?;

    let base_url = std::env::var("BASE_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string())
        .trim_end_matches('/')
        .to_string();

    let state = AppState { store: Store::new(pool), base_url };

    let app = Router::new()
        .route("/health", get(router::AppRouter::health))
        .route("/shortened", post(router::AppRouter::shortened))
        .route("/{code}", get(router::AppRouter::redirect))
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("Listening on {port}");
    axum::serve(listener, app).await?;
    Ok(())
}
