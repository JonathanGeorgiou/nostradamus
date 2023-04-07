use axum::{
    routing::get,
    Router,
};
use diesel::{PgConnection, Connection};
use dotenvy::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use  std::net::SocketAddr;

mod models;
// mod schema;
mod controllers;


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

    establish_connection(db_url);

    let app = Router::new()
        .route("/", get(|| async {"hello world"}));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn establish_connection(database_url: String)-> PgConnection {
    dotenv().ok();
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to database {}", database_url))
}
