use serde::{Deserialize, Serialize};

/// Database representation of an Event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Event {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub creator_user_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of an New Event
#[derive(Deserialize, Debug)]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub participants: Vec<i32>,
}

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

/// Representation of a new EventTeam
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewEventTeam {
    pub name: String,
}

// Representation of a new EventTeamParticipant
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventTeamParticipant {
    pub event_team_id: i32,
    pub event_participant_id: i32,
    pub created_at: chrono::NaiveDateTime,
}
