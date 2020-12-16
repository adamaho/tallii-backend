use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::services::users::models::PublicUser;

pub struct FriendsTable;

impl FriendsTable {
    /// Gets a list of followers of the provided username
    pub async fn get_followers_by_id(pool: &PgPool, user_id: &i32) -> Result<Vec<PublicUser>, TalliiError> {
        let followers = sqlx::query_as::<_, PublicUser>(
            r#"
                select
                    u.user_id,
                    u.avatar,
                    u.username,
                    u.taunt
                from
                    friends f
                inner join
                    users u
                on
                    f.user_id = u.user_id
                where
                    f.friend_user_id = $1
            "#
        )
            .bind(user_id)
            .fetch_all(pool)
            .await?;

        Ok(followers)
    }

    /// Gets a list of users that the username is following
    pub async fn get_following_by_id(pool: &PgPool, user_id: &i32) -> Result<Vec<PublicUser>, TalliiError> {
        let followers = sqlx::query_as::<_, PublicUser>(
            r#"
                select
                    u.user_id,
                    u.avatar,
                    u.username,
                    u.taunt
                from
                    friends f
                inner join
                    users u
                on
                    f.friend_user_id = u.user_id
                where
                    f.user_id = $1
            "#
        )
            .bind(user_id)
            .fetch_all(pool)
            .await?;

        Ok(followers)
    }

    /// Follows a user based on the provided username
    pub async fn follow_user_by_id(pool: &PgPool, user_id: &i32, friend_user_id: &i32) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                insert into
                    friends (user_id, friend_user_id, state)
                values
                    ($1, $2, 'active')
            "#
        )
            .bind(user_id)
            .bind(friend_user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Unfollows a user based on the provided username
    pub async fn unfollow_user_by_id(pool: &PgPool, user_id: &i32, friend_user_id: &i32) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                delete from
                    friends f
                where
                    f.user_id = $1
                    and f.friend_user_id = $2
            "#
        )
            .bind(user_id)
            .bind(friend_user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}