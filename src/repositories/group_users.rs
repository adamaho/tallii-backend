use futures::future::try_join_all;

use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::models::group_user::{GroupUser, NewGroupUser};
use crate::services::AuthenticatedUser;

pub struct GroupUsersRepository;

impl GroupUsersRepository {
    /// Creates a new group in the database
    pub async fn create_group_users(
        pool: &PgPool,
        _creator: &AuthenticatedUser,
        group_id: &i32,
        group_users: &Vec<NewGroupUser>,
    ) -> Result<Vec<GroupUser>, TalliiError> {
        // create an empty vector for the queries
        let mut users_to_insert = Vec::new();

        // create the queries for each of the new members and add them to the vector
        for user in group_users {
            users_to_insert.push(
                sqlx::query_as::<_, GroupUser>(
                    "insert into groups_users (group_id, user_id, user_type) values ($1, $2, $3) returning *"
                )
                    .bind(group_id)
                    .bind(&user.user_id)
                    .bind(&user.user_type)
                    .fetch_one(pool)
            )
        }

        // create all of the new members
        let created_group_users = try_join_all(users_to_insert).await?;

        Ok(created_group_users)
    }
}
