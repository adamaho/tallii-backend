use serde::{Deserialize, Serialize};

/// Database representation of an Event
#[derive(sqlx::FromRow, Deserialize, Serialize)]
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

/// Database representation of an EventTeam
#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct EventTeam {
    pub event_team_id: i32,
    pub event_id: i32,
    pub name: String,
    pub score: i32,
    pub winner: bool,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a new EventTeam
#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewEventTeam {
    pub event_id: i32,
    pub name: String,
}

/// Database representation of an EventTeamMember
#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct EventTeamMember {
    pub event_team_member_id: i32,
    pub event_team_id: i32,
    pub user_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a new EventTeamMember
#[derive(Deserialize)]
pub struct NewEventTeamMember {
    pub event_team_id: i32,
    pub user_id: i32,
}

/// Database representation of an EventTag
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct EventTag {
    pub event_tag_id: i32,
    pub event_id: i32,
    pub tag_id: i32,
}

/// Database representation of a new EventTag
#[derive(Deserialize)]
pub struct NewEventTag {
    pub event_id: i32,
    pub tag_id: i32,
}

/// Request body for creating a new event
#[derive(Deserialize)]
pub struct NewEventRequest {
    pub event: NewEvent,
    pub teams: Vec<NewEventTeamRequest>,
    pub tags: Vec<NewEventTag>
}

/// Request body shape for creating a new event team
#[derive(Deserialize)]
pub struct NewEventTeamRequest {
    pub team: NewEventTeam,
    pub members: Vec<NewEventTeamMember>
}

