use serde::{Deserialize, Serialize};

/// Database representation of a User
#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub user_id: i32,
    pub avatar: Option<String>,
    pub email: String,
    pub password: String,
    pub invite_code: String,
    pub username: String,
    pub taunt: Option<String>,
    pub verified: Option<bool>,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a user that can be publicized
#[derive(sqlx::FromRow, Serialize)]
pub struct MeUser {
    pub user_id: i32,
    pub avatar: Option<String>,
    pub email: String,
    pub username: String,
    pub verified: Option<bool>,
    pub taunt: Option<String>,
}

/// Representation of a user that can be publicized
#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct PublicUser {
    pub user_id: i32,
    pub avatar: Option<String>,
    pub username: String,
    pub taunt: Option<String>,
}

/// Representation of a New User
#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub email: String,
    pub invite_code: String,
    pub password: String,
    pub username: String,
}

/// Representation of an User to Update
#[derive(Debug, Deserialize)]
pub struct EditUser {
    pub avatar: String,
    pub taunt: String,
}

/// Representation of an User to Login
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

/// Database representation of an InviteCode
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct InviteCode {
    pub id: String,
}

/// Representation of struct for creating InviteCodes
#[derive(Debug, Deserialize)]
pub struct CreateInviteCode {
    pub amount: i32,
}

/// Representation of the search query
#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub username: String,
}
