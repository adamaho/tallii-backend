use serde::Serialize;

/// Database representation of a Friend
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Friend {
    pub user_id: String,
    pub friend_user_id: String,
    pub state: String,
    pub created_at: chrono::NaiveDateTime,
}
