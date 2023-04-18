use axum::{Router, routing::MethodRouter};
use serde_json::to_string_pretty;
use sqlx::{PgPool, query_as, types::Json};

use crate::{models::models::Player, error::AppError};

#[derive(Debug)]
struct Row {
    id: i32,
    player: Json<Player>,
}

pub async fn list_players(pool: &PgPool) -> Vec<Player> {
    let result: Vec<Player> = query_as!(
    Player,
        "select * from player",
).fetch_all(&pool).await.expect("unable to fetch players");
    return result

}


fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}

