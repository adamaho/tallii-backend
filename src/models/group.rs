use serde::{Deserialize, Serialize};

use crate::models::group_user::{NewGroupUser, GroupUser};

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
    pub members: Vec<NewGroupUser>,
}

/// Representation of an Group to Update
#[derive(Debug, Deserialize)]
pub struct EditGroup {
    pub avatar: String,
    pub taunt: String,
}

/// Representation of a GroupResponsePayload
#[derive(Debug, Serialize)]
pub struct GroupResponsePayload {
    pub group_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub members: Vec<GroupUser>,
    pub created_at: chrono::NaiveDateTime,
}
