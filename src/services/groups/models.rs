use serde::{Deserialize, Serialize};

/// Database representation of a Group
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Group {
    pub group_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a New Group
#[derive(Debug, Deserialize)]
pub struct NewGroup {
    pub name: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub members: Vec<NewGroupMember>,
}

/// Representation of an Group to update
#[derive(Debug, Deserialize)]
pub struct EditGroup {
    pub group_id: i32,
    pub avatar: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Representation of a GroupResponsePayload and is returned when a group is created
#[derive(Debug, Serialize)]
pub struct GroupResponsePayload {
    pub group_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub members: Vec<GroupMember>,
}

/// Database representation of a Group Member
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct GroupMember {
    pub group_id: i32,
    pub user_id: i32,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a Group Member
#[derive(Debug, Serialize, Deserialize)]
pub struct NewGroupMember {
    pub user_id: i32,
    pub role: String,
}

/// Representation of an Group Member
#[derive(Debug, Deserialize)]
pub struct EditGroupMember {
    pub user_id: i32,
    pub role: String,
}

