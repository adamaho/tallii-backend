use std::sync::Arc;

use futures::future::try_join_all;
use nanoid::generate;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;

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
    pub async fn get_by_id(&self, id: &String) -> Result<Option<InviteCode>, TalliiError> {
        let invite_code =
            sqlx::query_as::<_, InviteCode>("select * from invite_codes where id = $1")
                .bind(id)
                .fetch_optional(&*self.pool)
                .await?;

        Ok(invite_code)
    }

    /// Gets all invite codes
    /// TODO: Hide this behind the citadel
    pub async fn get_all(&self) -> Result<Vec<InviteCode>, TalliiError> {
        let all_invite_codes = sqlx::query_as::<_, InviteCode>("select * from invite_codes")
            .fetch_all(&*self.pool)
            .await?;

        Ok(all_invite_codes)
    }

    /// Creates invite codes for the specified amount
    /// Note, this query may not be the most performant query
    /// because it is not a bulk insert but that is fine.
    /// TODO: Hide this behind the citadel
    pub async fn create_many(&self, amount: i32) -> Result<(), TalliiError> {
        let mut new_codes_queries = Vec::new();

        // generate invite codes for the given amount
        for _ in 0..amount {
            let code = generate(6);

            new_codes_queries.push(
                sqlx::query("insert into invite_codes (id) values ($1)")
                    .bind(code)
                    .execute(&*self.pool),
            )
        }

        try_join_all(new_codes_queries).await?;

        Ok(())
    }
}
