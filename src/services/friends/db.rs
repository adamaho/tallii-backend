use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::services::friends::models::{FriendRequest, FriendRequestAcceptance};
use crate::services::auth::AuthenticatedUser;

pub struct FriendRepository;

impl FriendRepository {

    // Gets a list of friends
    // pub async fn get_many(
    //     pool: &PgPool,
    //     user: &AuthenticatedUser
    // ) -> Result<FriendResponse, TalliiError> {
    //
    // }

    /// Creates a friend invite in the database
    pub async fn create_friend_request(
        pool: &PgPool,
        new_friend: &FriendRequest,
        user: &AuthenticatedUser,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            "insert into friends (user_id, friend_id, friend_status) values ($1, $2, 'invited')",
        )
        .bind(&user.user_id)
        .bind(&new_friend.friend_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Accepts a friend invite in the database
    pub async fn accept_friend_request(
        pool: &PgPool,
        new_friend: &FriendRequestAcceptance,
        user: &AuthenticatedUser,
    ) -> Result<(), TalliiError> {
        // create row for the friend acceptance
        sqlx::query(
            "insert into friends (user_id, friend_id, friend_status) values ($1, $2, 'friend')",
        )
        .bind(&user.user_id)
        .bind(&new_friend.user_id)
        .execute(pool)
        .await?;

        // modify the existing row to change friend_status
        sqlx::query("update friends set friend_status = 'friend' where user_id = $1")
            .bind(&new_friend.user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
