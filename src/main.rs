pub mod store;
pub mod shortened_response;
pub mod router;
pub mod shortened_request;

use sqlx::postgres::PgPoolOptions;
use axum::{routing::get, routing::post, Router};
use crate::router::AppRouter;
use crate::store::Store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&url).await?;

    let store: Store = Store::new(pool);
    let app_router = AppRouter {};
    let app = Router::new()
        .route("/health", get(AppRouter::health))
        .route("/shortened", post(AppRouter::shortened))
        .route("/{code}", get(AppRouter::redirect))
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on :3000");
    axum::serve(listener, app).await?;
    Ok(())
}
