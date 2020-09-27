use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::PgPool;
use sqlx::Transaction;

use crate::errors::TalliiError;
use crate::models::group_user::{GroupUser, NewGroupUser};
use crate::services::AuthenticatedUser;

pub struct GroupUsersRepository;

impl GroupUsersRepository {
    /// Creates a group_users in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        group_id: i32,
        group_users: &Vec<NewGroupUser>,
    ) -> Result<Vec<GroupUser>, TalliiError> {
        // create a string with the query
        let mut query =
            String::from("insert into groups_users (group_id, user_id, user_type) values ");

        // create the queries for each of the new members and add them to the vector
        for (i, user) in group_users.iter().enumerate() {
            query.push_str(&format!(
                "({}, {}, '{}')",
                group_id, user.user_id, user.user_type
            ));

            // if we are appending values onto the query we need to separate them with commas
            if i < group_users.len() - 1 {
                query.push_str(",")
            }
        }

        // create all of the new members
        let created_group_users = sqlx::query_as::<_, GroupUser>(&format!("{} returning *", query))
            .fetch_all(tx)
            .await?;

        Ok(created_group_users)
    }

    ///  Checks if the requesting user has the permission to modify the group
    pub async fn check_ownership(
        pool: &PgPool,
        user: &AuthenticatedUser,
        group_id: i32,
    ) -> Result<bool, TalliiError> {
        // query
        let member = sqlx::query_as::<_, GroupUser>("select * from groups_users where group_id = $1 and user_id = $2 and user_type = 'owner'")
            .bind(group_id)
            .bind(user.user_id)
            .fetch_optional(pool)
            .await?;

        // if a row isn't returned then user isnt a part of the group or is not an owner
        if let None = member {
            return Ok(false);
        }

        // user is owner
        Ok(true)
    }
}
