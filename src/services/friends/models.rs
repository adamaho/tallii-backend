use serde::{Deserialize, Serialize};

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
#[serde(rename_all(serialize="camelCase"))]
pub struct Friend {
    pub user_id: i32,
    pub friend_id: i32,
    pub friend_status: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
#[serde(rename_all(serialize="camelCase"))]
pub struct FriendResponse {
    pub user_id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub taunt: Option<String>,
}

/// Representation of a New Friend
#[derive(Debug, Deserialize)]
#[serde(rename_all(serialize="camelCase"))]
pub struct FriendRequest {
    pub friend_id: i32,
}
/// Representation of a New Friend
#[derive(Debug, Deserialize)]
#[serde(rename_all(serialize="camelCase"))]
pub struct FriendRequestAcceptance {
    pub user_id: i32,
}

/// Representation of an Friend to update
#[derive(Debug, Deserialize)]
#[serde(rename_all(serialize="camelCase"))]
pub struct EditFriend {
    pub user_id: i32,
    pub friend_id: i32,
    pub friend_status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}
