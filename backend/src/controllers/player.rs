use serde_json::to_string_pretty;
use sqlx::{PgPool, query_as, types::Json};

use crate::{models::models::Player, error::AppError};

#[derive(Debug)]
struct Row {
    id: i32,
    player: Json<Player>,
}

pub async fn list_players(pool: &PgPool) -> anyhow::Result<()> {
    let rows = query_as!(
    Row,
        r#"
        SELECT id, player as "player: Json<Player>"
        FROM PLAYER
        "#
    ).fetch_all(pool)
        .await?;

    for row in rows {
        println!(
        "{}: {}",
            row.id,
            &to_string_pretty(&row.player)?
        );
    }

    Ok(())
}
