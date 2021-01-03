use serde::{Deserialize, Serialize};

/// Representation of a member of an event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventMember {
    pub member_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub state: String,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a member to update
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateMemberRequest {
    pub state: String,
    pub role: String,
}

/// Representation of a member response payload
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventMemberResponse {
    pub user_id: i32,
    pub username: String,
    pub emoji: String,
    pub bg_color: String,
    pub bio: Option<String>,
}

/// Representation of a member to update
#[derive(Deserialize, Serialize, Debug)]
pub struct InviteMemberRequest {
    pub user_id: i32,
}

/// Represents Member Existing
#[derive(sqlx::FromRow, Debug)]
pub struct MemberExists {
    pub exists: bool,
}
