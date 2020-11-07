use serde::{Deserialize, Serialize};

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Friend {
    pub user_id: i32,
    pub friend_id: i32,
    pub friend_status: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct FriendResponse {
    pub user_id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub taunt: Option<String>,
}

/// Representation of a New Friend
#[derive(Debug, Deserialize)]
pub struct FriendRequest {
    pub username: String,
}

/// Representation of a New Friend
#[derive(Debug, Deserialize)]
pub struct FriendRequestAcceptance {
    pub user_id: i32,
}

/// Representation of a New Friend
#[derive(Debug, Deserialize)]
pub struct FriendRequestDeny {
    pub user_id: i32,
}

/// Representation of an Friend to update
#[derive(Debug, Deserialize)]
pub struct EditFriend {
    pub user_id: i32,
    pub friend_id: i32,
    pub friend_status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
