use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::friends::models::{FriendRequest, FriendRequestAcceptance, FriendResponse};
use crate::services::users::models::User;

pub struct FriendRepository;

impl FriendRepository {
    /// Gets a list of friends
    pub async fn get_many(
        pool: &PgPool,
        user: &AuthenticatedUser,
    ) -> Result<Vec<FriendResponse>, TalliiError> {
        let friends = sqlx::query_as::<_, FriendResponse>(
            r#"
                select users.user_id, users.username, users.avatar, users.taunt
                from friends inner join users on users.user_id = friends.friend_id
                where friends.user_id = $1 and friend_status = 'friend'
            "#,
        )
        .bind(user.user_id)
        .fetch_all(pool)
        .await?;
        Ok(friends)
    }

    /// Gets a list of friend requests where the current user is the requester
    pub async fn get_many_requests(
        pool: &PgPool,
        user: &AuthenticatedUser,
    ) -> Result<Vec<FriendResponse>, TalliiError> {
        let requests = sqlx::query_as::<_, FriendResponse>(
            r#"
                select users.user_id, users.username, users.avatar, users.taunt
                from friends inner join users on users.user_id = friends.friend_id
                where friends.user_id = $1 and friend_status = 'requested'
            "#,
        )
        .bind(user.user_id)
        .fetch_all(pool)
        .await?;

        Ok(requests)
    }

    /// Gets a list of friend invitations where the current user is the invitee
    pub async fn get_many_invitations(
        pool: &PgPool,
        user: &AuthenticatedUser,
    ) -> Result<Vec<FriendResponse>, TalliiError> {
        let requests = sqlx::query_as::<_, FriendResponse>(
            r#"
                select users.user_id, users.username, users.avatar, users.taunt
                from friends inner join users on users.user_id = friends.user_id
                where friends.friend_id = $1 and friend_status = 'requested'
            "#,
        )
        .bind(user.user_id)
        .fetch_all(pool)
        .await?;

        Ok(requests)
    }

    /// Creates a friend invite in the database
    pub async fn create_friend_request(
        pool: &PgPool,
        new_friend: &FriendRequest,
        user: &AuthenticatedUser,
    ) -> Result<(), TalliiError> {
        // fetch the friend to add by username
        let user_to_add = sqlx::query_as::<_, User>("select * from users where username = $1")
            .bind(&new_friend.username)
            .fetch_optional(pool)
            .await?;

        // if there is no user with that username return an error
        if user_to_add.is_none() {
            // TODO: Move to invalid request or something
            return Err(TalliiError::INTERNAL_SERVER_ERROR.default());
        }

        // create the new friend request
        sqlx::query(
            "insert into friends (user_id, friend_id, friend_status) values ($1, $2, 'requested')",
        )
        .bind(&user.user_id)
        .bind(user_to_add.unwrap().user_id)
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
        sqlx::query(
            "update friends set friend_status = 'friend' where user_id = $1 and friend_id = $2",
        )
        .bind(&new_friend.user_id)
        .bind(&user.user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
