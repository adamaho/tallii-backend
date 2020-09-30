use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::PgPool;
use sqlx::Transaction;

use crate::errors::TalliiError;
use crate::models::group_member::{GroupMember, NewGroupMember};
use crate::services::AuthenticatedUser;

pub struct GroupMembersRepository;

impl GroupMembersRepository {
    /// Creates a group_users in the database
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        user: &AuthenticatedUser,
        group_id: i32,
        group_users: &Vec<NewGroupMember>,
    ) -> Result<Vec<GroupMember>, TalliiError> {
        // create a string with the query
        let mut query =
            String::from("insert into groups_members (group_id, user_id, role) values ");

        // add the user that is creating the group as the owner
        query.push_str(&format!("({}, {}, 'owner'), ", group_id, user.user_id));

        // create the queries for each of the new members and add them to the vector
        for (i, user) in group_users.iter().enumerate() {
            query.push_str(&format!(
                "({}, {}, '{}')",
                group_id, user.user_id, user.role
            ));

            // if we are appending values onto the query we need to separate them with commas
            if i < group_users.len() - 1 {
                query.push_str(",")
            }
        }

        // create all of the new members
        let created_group_users =
            sqlx::query_as::<_, GroupMember>(&format!("{} returning *", query))
                .fetch_all(tx)
                .await?;

        Ok(created_group_users)
    }

    /// Creates a group_users in the database
    pub async fn create_one(
        pool: &PgPool,
        group_id: i32,
        group_member: &NewGroupMember,
    ) -> Result<(), TalliiError> {
        // create a new member
        sqlx::query("insert into groups_members (group_id, user_id, role) values $1, $2, $3")
            .bind(group_id)
            .bind(&group_member.user_id)
            .bind(&group_member.role)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Gets all members of the provided group
    pub async fn get_many(pool: &PgPool, group_id: i32) -> Result<Vec<GroupMember>, TalliiError> {
        let all_members =
            sqlx::query_as::<_, GroupMember>("select * from group_members where group_id = $1")
                .bind(group_id)
                .fetch_all(pool)
                .await?;

        Ok(all_members)
    }

    ///  Checks if the requesting user has the permission to modify the group
    pub async fn check_ownership(
        pool: &PgPool,
        user: &AuthenticatedUser,
        group_id: i32,
    ) -> Result<bool, TalliiError> {
        // query
        let member = sqlx::query_as::<_, GroupMember>(
            "select * from groups_members where group_id = $1 and user_id = $2 and role = 'owner'",
        )
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

    ///  Checks if the requesting user is a part of the group
    pub async fn check_membership(
        pool: &PgPool,
        user: &AuthenticatedUser,
        group_id: i32,
    ) -> Result<bool, TalliiError> {
        // query
        let member = sqlx::query_as::<_, GroupMember>(
            "select role from groups_members where group_id = $1 and user_id = $2",
        )
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
