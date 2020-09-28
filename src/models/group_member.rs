use serde::{Deserialize, Serialize};

/// Database representation of a Group Member
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct GroupMember {
    pub group_id: i32,
    pub user_id: i32,
    pub user_type: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a Group Member
#[derive(Debug, Serialize, Deserialize)]
pub struct NewGroupMember {
    pub user_id: i32,
    pub user_type: String,
}

/// Representation of an Group Member
#[derive(Debug, Deserialize)]
pub struct EditGroupMember {
    pub user_type: String,
}
