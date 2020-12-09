use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::friends::models::{
    FriendQueryParams, FriendRequest, FriendResponse, FriendStatus,
};

use crate::services::users::models::User;

pub struct FriendsTable;

impl FriendsTable {
    /// Gets a list of friends
    pub async fn get_many(
        pool: &PgPool,
        params: &FriendQueryParams,
    ) -> Result<Vec<FriendResponse>, TalliiError> {
        // start the query
        let mut query = String::from(
            r#"
                select
                    users.user_id, users.username, users.avatar, users.taunt
                from
                    friends
                inner join
                    users on users.user_id = friends.friend_id
            "#,
        );

        // if there is a status query based on it
        if let Some(status) = &params.status {
            match status {
                FriendStatus::Invites => {
                    query.push_str("where friends.friend_id = $1 and friend_status = 'requested'");
                }
                FriendStatus::Requests => {
                    query.push_str("where friends.user_id = $1 and friend_status = 'requested'");
                }
                FriendStatus::Friends => {
                    query.push_str("where friends.user_id = $1 and friend_status = 'friend'");
                }
            }
        } else {
            query.push_str("where friends.user_id = $1 and friend_status = 'friend'");
        }

        // select the friends
        let friends = sqlx::query_as::<_, FriendResponse>(&query)
            .bind(params.user_id)
            .fetch_all(pool)
            .await?;

        Ok(friends)
    }

    /// Creates a friend invite in the database
    pub async fn create_friend_request(
        pool: &PgPool,
        new_friend: &FriendRequest,
        user: &AuthenticatedUser,
    ) -> Result<(), TalliiError> {
        // create the new friend request
        sqlx::query(
            "insert into friends (user_id, friend_id, friend_status) values ($1, $2, 'requested')",
        )
        .bind(&user.user_id)
        .bind(&new_friend.user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Accepts a friend invite in the database
    pub async fn accept_friend_request(
        pool: &PgPool,
        new_friend: &FriendRequest,
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
        sqlx::query(
            "update friends set friend_status = 'friend' where user_id = $1 and friend_id = $2",
        )
        .bind(&new_friend.user_id)
        .bind(&user.user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Denies a friend request that was received
    pub async fn deny_friend_request(
        pool: &PgPool,
        requested_user: &FriendRequest,
        user: &AuthenticatedUser,
    ) -> Result<(), TalliiError> {
        // delete row for the friend request
        sqlx::query("delete from friends where user_id = $1 and friend_id = $2")
            .bind(&requested_user.user_id)
            .bind(&user.user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Cancels a friend request that was sent
    pub async fn cancel_friend_request(
        pool: &PgPool,
        sent_friend: &FriendRequest,
        user: &AuthenticatedUser,
    ) -> Result<(), TalliiError> {
        // delete row for the friend request
        sqlx::query("delete from friends where friend_id = $1 and user_id = $2")
            .bind(&sent_friend.user_id)
            .bind(&user.user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
