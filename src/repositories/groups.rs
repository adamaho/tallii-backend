use std::sync::Arc;

use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::models::group::{Group, NewGroup};

pub struct GroupRepository {
    pool: Arc<PgPool>,
}

impl GroupRepository {
    /// Fetches a database pool connection to use for querying
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Creates a new group in the database
    pub async fn create_group(&self, group: &NewGroup) -> Result<Group, TalliiError> {
        let new_group = sqlx::query_as::<_, Group>(
            "insert into groups (name, description, avatar) values ($1, $2, $3) returning group_id, name, description, avatar, created_at"
        )
            .bind(&group.name)
            .bind(&group.description)
            .bind(&group.avatar)
            .fetch_one(&*self.pool)
            .await?;

        Ok(new_group)
    }
}
