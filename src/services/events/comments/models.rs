use serde::{Deserialize, Serialize};

/// Representation of a comment on an event
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct EventComment {
    pub comment_id: i32,
    pub event_id: i32,
    pub user_id: i32,
    pub comment: String,
    pub created_at: chrono::NaiveDateTime,
}

/// Representation comment to add to the database
#[derive(sqlx::FromRow, Deserialize, Serialize, Debug)]
pub struct CreateEventCommentRequest {
    pub comment: String,
}
