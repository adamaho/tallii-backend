use serde::{Deserialize, Serialize};

/// Database representation of an Event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
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
#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct NewEvent {
    pub group_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub event_type: String,
}

/// Query Params for an Event
#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct EventParams {
    pub group_id: i32,
    pub event_id: Option<i32>,
}

/// Database representation of an EventTeam
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
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
#[serde(rename_all(serialize="camelCase"))]
pub struct NewEventTeam {
    pub name: String,
}

/// Representation of a team to edit
#[derive(sqlx::FromRow, Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct EditEventTeam {
    pub event_team_id: i32,
    pub event_id: Option<i32>,
    pub name: Option<String>,
    pub score: Option<i32>,
    pub winner: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Query Params for the EventTeam's
#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct EventTeamParams {
    pub event_id: Option<i32>,
}

/// Database representation of an EventTeamMember
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct EventTeamMember {
    pub event_team_member_id: i32,
    pub event_team_id: i32,
    pub user_id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub taunt: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a new EventTeamMember
#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct NewEventTeamMember {
    pub user_id: i32,
}

/// Query Params for the EventTeamMember
#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct EventTeamMemberParams {
    pub event_id: Option<i32>,
}

/// Database representation of an EventTag
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct EventTag {
    pub event_tag_id: i32,
    pub event_id: i32,
    pub tag_id: i32,
}

/// Database representation of a new EventTag
#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct NewEventTag {
    pub event_id: i32,
    pub tag_id: i32,
}

/// Request body for creating a new event
#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct NewEventRequest {
    pub event: NewEvent,
    pub teams: Vec<NewEventTeamRequest>,
    // pub tags: Vec<NewEventTag>
}

/// Request body shape for creating a new event team
#[derive(Deserialize, Debug)]
#[serde(rename_all(serialize="camelCase"))]
pub struct NewEventTeamRequest {
    pub team: NewEventTeam,
    pub members: Vec<NewEventTeamMember>,
}
