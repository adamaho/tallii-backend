
/// Database representation of a Wager
#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Wager {
    pub wager_id: i32,
    pub home_user_id: i32,
    pub away_user_id: i32,
    pub team_id: i32,
    pub state: String,
    pub currency: i32,
    pub created_at: chrono::NaiveDateTime
}

/// Database representation of a Wager to insert
#[derive(sqlx::FromRow, Deserialize)]
pub struct NewWager {
    pub friend_user_id: i32,
    pub team_id: i32,
    pub state: String,
    pub currency: i32,
}

/// Database representation of a Wager to insert
#[derive(sqlx::FromRow, Deserialize)]
pub struct WagerResponse {
    pub wager_id: i32,

    pub friend_user_id: i32,
    pub team: i32,
    pub state: String,
    pub currency: i32,
}