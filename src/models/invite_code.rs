use serde::{Deserialize, Serialize};

/// Database representation of a User
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct InviteCode {
    pub id: String,
}
