use serde::{Deserialize, Serialize};

/// Database representation of a Group User
#[derive(sqlx::FromRow, Serialize)]
pub struct GroupUser {
    pub group_id: i32,
    pub user_id: i32,
    pub user_type: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a Group User
#[derive(Debug, Deserialize)]
pub struct NewGroupUser {
    pub group_id: i32,
    pub user_id: i32,
    pub user_type: String,
}

/// Representation of an Group User
#[derive(Debug, Deserialize)]
pub struct EditGroupUser {
    pub user_type: String,
}
