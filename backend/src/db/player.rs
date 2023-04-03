use super::{get_db_con, Result};
use crate::{error::Error::*, DBPool};
use common::*;
use mobc_postgres::tokio_postgres::Row;

pub const TABLE: &str = "player";
const SELECT_FIELDS: &str = "id, first_name, last_name, email_address, favourite_team";


// fetch players by their favourite team
pub async fn fetch_by_team(db_pool: &DBPool, favourite_team: String) -> Result<Vec<Player>> {
    let con = get_db_con(db_pool).await?;
    let query = format!(
    "SELECT {} FROM {} WHERE favourite_team = $1",
    SELECT_FIELDS, TABLE
    );
    let rows = con
        .query(query.as_str(), &[&favourite_team])
        .await
        .map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_player(&r)).collect())    
}

// create a player
pub async fn create(db_pool: &DBPool, body: PlayerRequest) -> Result<Player> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (name) VALUES ($1) RETURNING *", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.name])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_player(&row))
}

// convert returned database row to Player struct
fn row_to_player(row: &Row) -> Player {
    let id: i32 = row.get(0);
    let first_name: String = row.get(1);
    let last_name: String = row.get(2);
    let email_address: String = row.get(3);
    let favourite_team: String = row.get(4);
    Player { id, first_name, last_name, email_address, favourite_team }
}
