use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::friends::models::{
    Friend, FriendRequest, FriendRequestAcceptance, FriendResponse,
};

pub struct FriendRepository;

impl FriendRepository {
    // Gets a list of friends
    pub async fn get_many(
        pool: &PgPool,
        user: &AuthenticatedUser,
    ) -> Result<Vec<FriendResponse>, TalliiError> {
        let friends = sqlx::query_as::<_, FriendResponse>("select users.user_id, users.username, users.avatar, users.taunt from friends inner join users on users.user_id = friends.friend_id where friends.user_id = $1").bind(user.user_id).fetch_all(pool).await?;

        Ok(friends)
    }

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
