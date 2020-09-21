use serde::{Deserialize, Serialize};

use crate::models::group_user::NewGroupUser;

/// Database representation of a Group
#[derive(sqlx::FromRow, Serialize)]
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

/// Representation of an User to Update
#[derive(Debug, Deserialize)]
pub struct EditGroup {
    pub avatar: String,
    pub taunt: String,
}
