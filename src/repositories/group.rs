use std::sync::Arc;

use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;
use crate::models::group::{Group, NewGroup};

pub struct GroupRepository;

impl GroupRepository {
    const INSERT: &'static str = "insert into groups (name, description, avatar) values ($1, $2, $3) returning group_id, name, description, avatar, created_at";

    /// Creates a group in the database using the connection from the database pool
    pub async fn create_group(pool: &PgPool, new_group: &NewGroup) -> Result<Group, TalliiError> {
        let created_group = sqlx::query_as::<_, Group>(GroupRepository::INSERT)
            .bind(&new_group.name)
            .bind(&new_group.description)
            .bind(&new_group.avatar)
            .fetch_one(pool)
            .await?;

        Ok(created_group)
    }

    /// Creates a group in the database using the connection from the database pool
    pub async fn create_group_in_tx(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        new_group: &NewGroup,
    ) -> Result<Group, TalliiError> {
        let created_group = sqlx::query_as::<_, Group>(GroupRepository::INSERT)
            .bind(&new_group.name)
            .bind(&new_group.description)
            .bind(&new_group.avatar)
            .fetch_one(tx)
            .await?;

        Ok(created_group)
    }
}
