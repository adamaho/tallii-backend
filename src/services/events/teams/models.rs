use serde::{Deserialize, Serialize};

/// Database representation of an Team
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Team {
    pub team_id: i32,
    pub event_id: i32,
    pub name: String,
    pub score: i32,
    pub winner: bool,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a new Team
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewTeam {
    pub event_id: i32,
    pub name: String,
    pub players: Vec<i32>,
}

/// Representation of a player on a team
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct TeamPlayer {
    pub team_id: i32,
    pub player_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// Query params for getting players on a specific team
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct TeamPlayerQueryParams {
    pub event_id: i32
}

/// Query params for getting teams of a specific event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct TeamQueryParams {
    pub event_id: i32
}
