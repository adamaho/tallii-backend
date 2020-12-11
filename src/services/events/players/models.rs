use serde::{Deserialize, Serialize};

use crate::services::events::models::PlayerStatus;

/// Representation of a Player
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Player {
    pub player_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of an player to Update or Add
#[derive(Deserialize, Debug)]
pub struct PlayerRequest {
    pub status: PlayerStatus,
}

/// Query params for getting players
#[derive(Deserialize, Debug)]
pub struct PlayerQueryParams {
    pub event_id: i32,
}