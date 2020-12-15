use serde::{Deserialize, Serialize};

use crate::services::users::models::User;

/// Database representation of an Event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Event {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub creator_username: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Database representation of an Event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventResponse {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub creator: User,
    pub created_at: chrono::NaiveDateTime,
}

/// Database representation of an Event
#[derive(Serialize, Debug)]
pub struct CreatedEventResponse {
    pub event_id: i32,
}

/// Representation of an New Event
#[derive(Deserialize, Debug)]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub players: Vec<i32>,
}

/// Represents the Player Status to query for
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PlayerStatus {
    Pending,
    Declined,
    Accepted,
}

/// convert the values of the enum to a str
impl std::fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerStatus::Accepted => write!(f, "accepted"),
            PlayerStatus::Declined => write!(f, "declined"),
            PlayerStatus::Pending => write!(f, "pending"),
        }
    }
}

/// Represents Query Params for querying events of me
#[derive(Deserialize, Debug)]
pub struct MeEventQueryParams {
    pub player_status: Option<PlayerStatus>,
}

/// Represents Query Params for querying events of a user
#[derive(Deserialize, Debug)]
pub struct EventQueryParams {
    pub user_id: i32,
}
