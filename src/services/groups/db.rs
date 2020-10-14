use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::PgPool;
use sqlx::Transaction;

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::groups::models::{EditGroup, Group, GroupMember, NewGroup, NewGroupMember};

pub struct GroupRepository;

impl GroupRepository {
    /// Creates a group in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        new_group: &NewGroup,
    ) -> Result<Group, TalliiError> {
        let created_group = sqlx::query_as::<_, Group>(
            "insert into groups (name, description, avatar) values ($1, $2, $3) returning group_id, name, description, avatar, created_at"
        )
            .bind(&new_group.name)
            .bind(&new_group.description)
            .bind(&new_group.avatar)
            .fetch_one(tx)
            .await?;

        Ok(created_group)
    }

    /// Gets a group based on the user_id of the requesting user
    pub async fn get_by_user_id(
        pool: &PgPool,
        user: &AuthenticatedUser,
    ) -> Result<Vec<Group>, TalliiError> {
        let groups = sqlx::query_as::<_, Group>(
            r#"
                select
                groups.group_id, groups.name, groups.description, groups.avatar, groups.created_at
                from groups
                left join groups_members on groups.group_id = groups_members.group_id
                where user_id = $1;
            "#,
        )
        .bind(user.user_id)
        .fetch_all(pool)
        .await?;

        Ok(groups)
    }

    /// Updates a group
    pub async fn update(
        pool: &PgPool,
        group_id: i32,
        group: &EditGroup,
    ) -> Result<Group, TalliiError> {
        let updated_group = sqlx::query_as::<_, Group>("update groups set name = $1, description = $2, avatar = $3 where group_id = $4 returning *")
            .bind(&group.name)
            .bind(&group.description)
            .bind(&group.avatar)
            .bind(group_id)
            .fetch_one(pool)
            .await?;

        Ok(updated_group)
    }

    /// Deletes a group
    pub async fn delete(pool: &PgPool, group_id: i32) -> Result<(), TalliiError> {
        sqlx::query("delete from groups where group_id = $1")
            .bind(group_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}

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

        // create the queries for each of the new members and add them to the query string
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
            sqlx::query_as::<_, GroupMember>("select * from groups_members where group_id = $1")
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
        println!("{:?}", user);
        println!("{:?}", group_id);
        // query
        let member = sqlx::query_as::<_, GroupMember>(
            "select * from groups_members where group_id = $1 and user_id = $2",
        )
        .bind(group_id)
        .bind(user.user_id)
        .fetch_optional(pool)
        .await?;

        println!("{:?}", member);

        // if a row isn't returned then user isnt a part of the group
        if let None = member {
            return Ok(false);
        }

        Ok(true)
    }
}
