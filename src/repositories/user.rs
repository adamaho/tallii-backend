use std::sync::Arc;

use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::crypto::Crypto;
use crate::errors::TalliiError;
use crate::models::user::{NewUser, User, PublicUser};

#[derive(Debug)]
pub struct UserRepository {
    pool: Arc<PgPool>,
}

impl UserRepository {
    /// Fetches a database pool connection to use for querying
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Fetches a user with the provided email
    pub async fn get_by_email(&self, email: String) -> Result<Option<User>, TalliiError> {
        let user_with_email = sqlx::query_as::<_, User>(
            "select * from users where email = $1"
        )
        .bind(email)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(user_with_email)
    }

    /// Fetches a user that holds the provided invite code
    pub async fn get_by_invite_code(&self, invite_code: &str) -> Result<Option<PublicUser>, TalliiError> {
        let user_with_invite_code = sqlx::query_as::<_, PublicUser>(
            "select * from users where invite_code = $1"
        )
        .bind(invite_code)
        .fetch_optional(&*self.pool)
        .await?;

        Ok(user_with_invite_code)
    }

    /// Creates a user
    pub async fn create(&self, new_user: NewUser, crypto: &Crypto) -> Result<PublicUser, TalliiError> {

        // hash the password
        let hashed_password = crypto.hash_password(&new_user.password).await?;
        
        // create the user and return the public user
        let user = sqlx::query_as::<_, PublicUser>(
            "insert into users (email, password, invite_code, username) values ($1, $2, $3, $4) returning user_id, avatar, email, username, taunt",
        )
        .bind(new_user.email)
        .bind(hashed_password)
        .bind(new_user.invite_code)
        .bind(new_user.username)
        .fetch_one(&*self.pool)
        .await?;

        Ok(user)
    }
}
