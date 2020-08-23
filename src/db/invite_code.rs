use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::sync::Arc;

use crate::errors::TalliiError;
use crate::models::invite_code::InviteCode;

pub struct InviteCodeRepository {
    pool: Arc<PgPool>,
}

impl InviteCodeRepository {
    /// Fetches a database pool connection to use for querying
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    /// Gets an invite code by the provided id
    pub async fn get_by_id(
        &self,
        id: &String,
    ) -> Result<Option<InviteCode>, TalliiError> {
        let invite_code = sqlx::query_as::<_, InviteCode>("select * where id = $1")
            .bind(id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(invite_code)
    }
}
