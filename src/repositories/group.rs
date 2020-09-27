use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::PgPool;
use sqlx::Transaction;

use crate::errors::TalliiError;
use crate::models::group::{EditGroup, Group, NewGroup};
use crate::services::AuthenticatedUser;

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
                left join groups_users on groups.group_id = groups_users.group_id
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
