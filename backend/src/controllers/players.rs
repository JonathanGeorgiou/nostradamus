// use axum::{Extension, Json};


use axum::{extract::State, Json, http::StatusCode};
use deadpool_diesel::postgres::Pool;
// use diesel::{QueryDsl, SelectableHelper};
use diesel::prelude::*;
use crate::models::db_models::Player;
use crate::schema::player;

async fn list_players(State(pool): State<Pool>,) -> Result<Json<Vec<Player>>, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let res = conn.interact(
        |conn| player::table.select(Player::as_select())
        .load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}


/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
