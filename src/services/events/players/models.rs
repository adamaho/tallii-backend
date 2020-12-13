use serde::{Deserialize, Serialize};

/// Representation of a Player
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Player {
    pub player_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a player to update
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct UpdatePlayerRequest {
    pub status: String,
}

/// Representation of a PlayerResponse
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct PlayerResponse {
    pub player_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub taunt: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Query params for getting players
#[derive(Deserialize, Debug)]
pub struct PlayerQueryParams {
    pub event_id: i32,
}
