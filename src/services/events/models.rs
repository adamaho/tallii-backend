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

/// Database representation of an Event
#[derive(Serialize, Debug)]
pub struct CreatedEventResponse {
    pub event_id: i32,
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

/// Representation of an New Event
#[derive(Deserialize, Debug)]
pub struct NewEvent {
    pub name: String,
    pub description: Option<String>,
    pub participants: Vec<i32>,
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

/// Represents Query Params for querying an event
#[derive(Deserialize, Debug)]
pub struct EventQueryParams {
    pub user_id: String,
    pub player_status: Option<PlayerStatus>,
}
