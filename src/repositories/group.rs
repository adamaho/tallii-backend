use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::Transaction;

use crate::errors::TalliiError;
use crate::models::group::{Group, NewGroup};

pub struct GroupRepository;

impl GroupRepository {
    /// Creates a group in the database
    pub async fn create_group(
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
}
