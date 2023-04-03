use crate::{db, DBPool, Result};
use common::*;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn list_players_by_team_handler(favourite_team: String, db_pool: DBPool) -> Result<impl Reply> {
    let players = db::player::fetch_by_team(&db_pool, favourite_team)
        .await
        .map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &players.into_iter().map(PlayerResponse::of).collect(),
    ))
}
