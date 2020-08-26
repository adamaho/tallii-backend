use std::sync::Arc;

use actix_web::web;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::models::user::{NewUser, User};
use crate::models::invite_code::InviteCode;

#[derive(Debug)]
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
    pub async fn create(&self, new_user: NewUser) -> Result<(), TalliiError> {
        
        // check if the invite code is valid
        if let None = sqlx::query_as::<_, InviteCode>(
            "select * from invite_codes where id = $1"
        )
        .bind(&new_user.invite_code)
        .fetch_optional(&*self.pool)
        .await? {
            return Err(TalliiError::InvalidInviteCode);
        }

        // check if the invite code is being used by another user
        if let None = sqlx::query_as::<_, InviteCode>(
            "select * from users where invite_code = $1"
        )
        .bind(&new_user.invite_code)
        .fetch_optional(&*self.pool)
        .await? {
            return Err(TalliiError::InvalidInviteCode);
        }

        // // let password = new_user.password;

        // // hash the password in different thread to make sure we arent blocking
        // let foo = match web::block(move || {
        //     hash(new_user.password, DEFAULT_COST)
        // })
        // .await {
        //     Ok(thing) => thing,
        //     Err(err) => return Err(TalliiError::InternalServerError)
        // };
        
        // // create the user
        // let user = sqlx::query_as::<_, User>(
        //     "insert into users (email, password, invite_code, username) values ($1, $2, $3, $4) returning user_id, avatar, email, username, taunt",
        // )
        // .bind(new_user.email)
        // .bind(new_user.invite_code)
        // .bind(new_user.username)
        // .bind(foo)
        // .fetch_one(&*self.pool)
        // .await?;

        Ok(())
    }
}
