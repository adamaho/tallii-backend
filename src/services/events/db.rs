use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::PgPool;
use sqlx::Transaction;

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::events::models::{Event, NewEvent};

pub struct EventRespository;

impl EventRespository {
    /// Creates an event in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        new_event: &NewEvent,
        user: &AuthenticatedUser
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                insert into events (group_id, name, description, event_type, creator_user_id)
                values ($1, $2, $3, $4, $5)
            "#
        )
            .bind(&new_event.group_id)
            .bind(&new_event.name)
            .bind(&new_event.description)
            .bind(&new_event.event_type)
            .bind(&user.user_id)
            .execute(tx)
            .await?;

        Ok(())
    }
}