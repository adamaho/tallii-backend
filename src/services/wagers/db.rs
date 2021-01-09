use sqlx::PgPool;

use crate::errors::TalliiError;
use super::models::{Wager, NewWager};
use crate::services::auth::AuthenticatedUser;

pub struct WagersTable;

impl WagersTable {
    /// Gets wagers for user
    pub async fn get_wagers_by_user_id(pool: &PgPool, user: &AuthenticatedUser) {
        let wagers = sqlx::query_as::<_, Wager>(
            r#"

            "#
        )
            .bind(&user.user_id)
            .fetch_all(&pool)
            .await?;
    }
    /// Creates a user
    pub async fn create(
        pool: &PgPool,
        new_wager: &NewWager,
        user: &AuthenticatedUser
    ) -> Result<(), TalliiError> {

        // Create the wager for user that is creating the wager
        sqlx::query_as::<_, Wager>(
            "insert into wagers (home_user_id, away_user_id, team_id, state, currency) values ($1, $2, $3, $4, $5)",
        )
            .bind()
            .bind(hashed_password)
            .bind(&avatar.emoji)
            .bind(&avatar.background)
            .bind(&new_user.invite_code)
            .bind(&new_user.username)
            .fetch_one(pool)
            .await?;

        Ok(())
    }
}