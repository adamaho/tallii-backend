use std::sync::Arc;

use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::models::user::User;

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    /// Fetches a database pool connection to use for querying
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Gets an user by user_id
    pub async fn get_by_id(
        &self,
        user_id: &i32,
    ) -> Result<User, TalliiError> {
        let user = sqlx::query_as::<_, User>("select user_id, username, email, taunt from users where user_id = $1")
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(user)
    }
}