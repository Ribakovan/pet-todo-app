use axum::{routing::get, Router};
use axum_error::Result;
use sqlx::sqlite::SqlitePool;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let dsn = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&dsn).await?;

    let app = Router::new()
        .route("/", get(index))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn index() -> String {
    format!("Hello world")
}
