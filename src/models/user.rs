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
