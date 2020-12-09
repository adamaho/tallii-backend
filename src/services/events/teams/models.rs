use serde::{Deserialize, Serialize};

/// Database representation of an EventTeam
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventTeam {
    pub event_team_id: i32,
    pub event_id: i32,
    pub name: String,
    pub score: i32,
    pub winner: bool,
    pub created_at: chrono::NaiveDateTime,
}

/// Database representation of an EventTeam row
#[derive(sqlx::FromRow, Deserialize, Serialize, Clone, Debug)]
pub struct EventTeamRow {
    pub event_team_id: i32,
    pub event_id: i32,
    pub name: String,
    pub score: i32,
    pub winner: bool,
    pub created_at: chrono::NaiveDateTime,
    pub event_player_id: i32,
}

/// Representation of a new EventTeam
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewEventTeam {
    pub name: String,
    pub players: Vec<i32>,
}

/// Representation of a new EventTeamParticipant
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventTeamPlayer {
    pub event_team_id: i32,
    pub event_player_id: i32,
    pub created_at: chrono::NaiveDateTime,
}
