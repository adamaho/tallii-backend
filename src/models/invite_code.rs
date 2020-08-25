use serde::{Deserialize, Serialize};

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
