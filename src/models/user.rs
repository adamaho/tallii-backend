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
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a user that can be publicized
#[derive(sqlx::FromRow, Serialize)]
pub struct PublicUser {
    pub user_id: i32,
    pub avatar: Option<String>,
    pub email: String,
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
