use serde::Serialize;

/// Database representation of a User
#[derive(sqlx::FromRow, Serialize)]
pub struct InviteCode {
    id: String
}