use axum::{
    routing::get,
    Router, Extension,
};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use  std::net::SocketAddr;

mod models;
mod schema;
mod controllers;
mod error;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("unable to connect to database");

    let app = Router::new()
        .route("/", get(|| async {"hello world"}))
        .route("/api/players", get(controllers::player::list_players(&pool)))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


