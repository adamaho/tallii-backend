use serde::{Deserialize, Serialize};

/// Database representation of an Event
#[derive(sqlx::FromRow, Serialize)]
pub struct Event {
    pub event_id: i32,
    pub group_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub event_type: String,
    pub creator_user_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of an New Event
#[derive(Deserialize)]
pub struct NewEvent {
    pub group_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub event_type: String,
}