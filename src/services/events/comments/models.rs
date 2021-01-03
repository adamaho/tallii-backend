use serde::{Deserialize, Serialize};
use crate::services::users::models::PublicUser;

/// Representation of a comment on an event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventComment {
    pub comment_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub comment: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation of a comment row
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventCommentRow {
    pub comment_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub comment: String,
    pub created_at: chrono::NaiveDateTime,
    pub emoji: String,
    pub bg_color: String,
    pub username: String,
    pub bio: Option<String>
}

/// Representation of a comment row
#[derive(Deserialize, Serialize, Debug)]
pub struct EventCommentResponse {
    pub comment_id: i32,
    pub event_id: i32,
    pub user: PublicUser,
    pub comment: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation comment to add to the database
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct CreateEventCommentRequest {
    pub comment: String,
}
