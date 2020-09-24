use serde::{Deserialize, Serialize};

/// Database representation of a Group User
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct GroupUser {
    pub group_id: i32,
    pub user_id: i32,
    pub user_type: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a Group User
#[derive(Debug, Serialize, Deserialize)]
pub struct NewGroupUser {
    pub user_id: i32,
    pub user_type: String,
}

/// Representation of an Group User
#[derive(Debug, Deserialize)]
pub struct EditGroupUser {
    pub user_type: String,
}
