use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::Transaction;

use crate::errors::TalliiError;
use crate::models::group_user::{GroupUser, NewGroupUser};

pub struct GroupUsersRepository;

impl GroupUsersRepository {
    /// Creates a new group in the database
    pub async fn create_group_users(
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
}
