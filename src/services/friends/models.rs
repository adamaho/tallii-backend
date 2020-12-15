use serde::{Deserialize, Serialize};

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Friend {
    pub user_id: String,
    pub friend_user_id: String,
    pub state: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct FriendResponse {
    pub user_id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub taunt: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

/// Represents the operation to perform for the request
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FriendOperation {
    SendRequest,
    AcceptRequest,
    CancelRequest,
    DenyRequest,
}

/// Represents the post request body of a friend
#[derive(Debug, Deserialize)]
pub struct FriendRequest {
    pub user_id: i32,
    pub operation: FriendOperation,
}

/// Query Params for FriendQueryParams
#[derive(Debug, Deserialize)]
pub struct FriendQueryParams {
    pub user_id: i32,
}

/// Represents the Friend Status to query for
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MeFriendStatus {
    Invited,
    Requested,
    Accepted,
    Blocked,
}

/// Query Params for getting me friend
#[derive(Debug, Deserialize)]
pub struct MeFriendQueryParams {
    pub status: Option<MeFriendStatus>,
}
