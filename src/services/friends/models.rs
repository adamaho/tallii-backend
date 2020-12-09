use serde::{Deserialize, Serialize};

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Friend {
    pub user_id: i32,
    pub friend_id: i32,
    pub friend_status: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct FriendCount {
    pub count: i64,
}

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct FriendResponse {
    pub user_id: i32,
    pub username: String,
    pub avatar: Option<String>,
    pub taunt: Option<String>,
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

/// Representation of an Friend to update
#[derive(Debug, Deserialize)]
pub struct EditFriend {
    pub user_id: i32,
    pub friend_id: i32,
    pub friend_status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Represents the Friend Status to query for
#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FriendStatus {
    Pending,
    Accepted,
    Blocked,
}

/// Query Params for FriendQueryParams
#[derive(Debug, Deserialize)]
pub struct FriendQueryParams {
    pub user_id: i32,
    pub status: Option<FriendStatus>,
}
