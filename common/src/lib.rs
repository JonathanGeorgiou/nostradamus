use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Player {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email_address: Option<String>,
    pub favourite_team: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlayerRequest {
    pub first_name: String,
    pub last_name: String,
    pub email_address: Option<String>,
    pub favourite_team: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PlayerResponse {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email_address: Option<String>,
    pub favourite_team: Option<String>,
}

impl PlayerResponse {
    pub fn of(player: Player) -> PlayerResponse {
        PlayerResponse {
            id: player.id,
            first_name: player.first_name,
            last_name: player.last_name,
            email_address: player.email_address,
            favourite_team: player.favourite_team,
        }
    } 
}
