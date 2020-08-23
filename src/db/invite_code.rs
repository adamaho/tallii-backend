use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use std::sync::Arc;

use crate::errors::TalliiErrorCode;
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
        provided_code: &InviteCode,
    ) -> Result<InviteCode, TalliiErrorCode> {
        let invite_code = sqlx::query_as::<_, InviteCode>("select * where id = $1")
            .bind(&provided_code.id)
            .fetch_one(&*self.pool)
            .await?;

        Ok(invite_code)
    }
}
