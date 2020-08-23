use serde::{Deserialize, Serialize};

/// Database representation of a User
#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    user_id: i32,
    avatar: Option<String>,
    email: String,
    password: String,
    invite_code: String,
    username: String,
    taunt: Option<String>,
    created_at: chrono::NaiveDateTime,
}

/// Representation of a New User
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub invite_code: String,
    pub password: String,
    pub username: String,
}

/// Representation of an User to Update
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub avatar: String,
    pub taunt: String,
}
