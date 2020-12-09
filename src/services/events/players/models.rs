use serde::{Deserialize, Serialize};

use crate::services::events::models::PlayerStatus;

/// Representation of an EventParticipant
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventPlayer {
    pub event_player_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of an EventParticipant to Update or Add
#[derive(Deserialize, Debug)]
pub struct EventPlayerRequest {
    pub user_id: i32,
    pub status: PlayerStatus,
}

/// Representation of a new EventParticipantRow
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventPlayerRow {
    pub event_player_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub taunt: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}
