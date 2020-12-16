use serde::{Deserialize, Serialize};

use crate::services::users::models::PublicUser;

/// Database representation of an Event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct Event {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub creator_user_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// Event row that is queried
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventRow {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub user_id: i32,
    pub avatar: Option<String>,
    pub username: String,
    pub taunt: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

/// Event response payload
#[derive(Deserialize, Serialize, Debug)]
pub struct EventResponse {
    pub event_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub creator: PublicUser,
    pub created_at: chrono::NaiveDateTime,
}

/// Update event request
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateEventRequest {
    pub name: String,
    pub description: Option<String>,
}

/// Database representation of an Event
#[derive(Serialize, Debug)]
pub struct CreatedEventResponse {
    pub event_id: i32,
}

/// Representation of an New Event
#[derive(Deserialize, Debug)]
pub struct CreateEventRequest {
    pub name: String,
    pub description: Option<String>,
    pub members: Vec<i32>,
}
