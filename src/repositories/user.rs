use std::sync::Arc;

use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::models::user::{NewUser, User};

pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    /// Fetches a database pool connection to use for querying
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Gets an user by user_id
    pub async fn get_by_id(&self, user_id: &i32) -> Result<User, TalliiError> {
        let user = sqlx::query_as::<_, User>(
            "select user_id, username, email, taunt from users where user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }

    /// Creates a user
    pub async fn create(&self, new_user: NewUser) -> Result<User, TalliiError> {
        // hash the password

        let user = sqlx::query_as::<_, User>(
            "insert into users (email, password, invite_code, username) values ($1, $2, $3, $4) returning user_id, avatar, email, username, taunt",
        )
        .bind(new_user.email)
        .bind(new_user.password)
        .bind(new_user.invite_code)
        .bind(new_user.username)
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }
}
