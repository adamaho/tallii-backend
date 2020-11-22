use crate::services::users::models::PublicUser;
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

/// Database representation of an Event query
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventRow {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub creator_user_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub user_id: i32,
    pub avatar: Option<String>,
    pub username: String,
}

/// Response payload for Event query
#[derive(Serialize, Debug)]
pub struct EventResponsePayload {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub creator: EventCreator,
    pub created_at: chrono::NaiveDateTime,
}

/// Represents a subset of user info for the creator of the event
#[derive(Serialize, Debug)]
pub struct EventCreator {
    pub user_id: i32,
    pub avatar: Option<String>,
    pub username: String,
}

/// Representation of an EventParticipant
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventParticipant {
    pub event_participant_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of an EventParticipant to Update or Add
#[derive(Deserialize, Debug)]
pub struct EventParticipantRequest {
    pub event_participant_id: Option<i32>,
    pub event_id: Option<i32>,
    pub user_id: i32,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Representation of a new EventParticipantRow
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventParticipantRow {
    pub event_participant_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub taunt: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of an New Event
#[derive(Deserialize, Debug)]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub participants: Vec<i32>,
}

/// Represents Query Params for querying an event
#[derive(Deserialize, Debug)]
pub struct EventQueryParams {
    pub participant_status: Option<String>,
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

/// Database representation of an EventTeam row
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventTeamRow {
    pub event_team_id: i32,
    pub event_id: i32,
    pub name: String,
    pub score: i32,
    pub winner: bool,
    pub created_at: chrono::NaiveDateTime,
    pub event_participant_id: i32,
}

/// Database representation of an EventTeam response
#[derive(Serialize, Debug)]
pub struct EventTeamResponse {
    pub event_team_id: i32,
    pub event_id: i32,
    pub name: String,
    pub score: i32,
    pub winner: bool,
    pub participants: Vec<EventTeamParticipantResponse>,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a new EventTeam
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct NewEventTeam {
    pub name: String,
    pub participants: Vec<i32>,
}

/// Representation of a new EventTeamParticipant
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventTeamParticipant {
    pub event_team_id: i32,
    pub event_participant_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a new EventTeamParticipant response
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventTeamParticipantResponse {
    pub event_participant_id: i32,
}
