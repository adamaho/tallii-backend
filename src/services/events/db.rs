use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;

use crate::services::events::models::{
    Event, EventQueryParams, MeEventQueryParams, NewEvent, PlayerStatus,
};

pub struct EventsTable;

impl EventsTable {
    // initial part of the query for getting events
    fn get_event_query() -> String {
        String::from(
            r#"
                select
                    events.event_id
                    events.name
                    events.description
                    events.creator_user_id
                    events.created_at
                from
                    events
                left join
                    events_players ep
                on
                    events.event_id = ep.event_id
                left join
                    users u
                on
                    events.creator_user_id = u.user_id
            "#,
        )
    }

    /// Creates an event in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        new_event: &NewEvent,
        user: &AuthenticatedUser,
    ) -> Result<Event, TalliiError> {
        let event = sqlx::query_as::<_, Event>(
            r#"
                insert
                    into
                events
                    (name, description, creator_user_id)
                values
                    ($1, $2, $3)
                returning
                    *
            "#,
        )
        .bind(&new_event.name)
        .bind(&new_event.description)
        .bind(&user.user_id)
        .fetch_one(tx)
        .await?;

        Ok(event)
    }

    /// Gets a single event from the database
    pub async fn get_one(pool: &PgPool, event_id: &i32) -> Result<Event, TalliiError> {
        // get the initial query
        let mut query = Self::get_event_query();

        // push the predicate
        query.push_str("where events.event_id = $1");

        // execute the query
        let event = sqlx::query_as::<_, Event>(&query)
            .bind(event_id)
            .fetch_one(pool)
            .await?;

        // return the result
        Ok(event)
    }

    /// Gets all Events that a user has accepted
    pub async fn get_many(
        pool: &PgPool,
        params: &EventQueryParams,
    ) -> Result<Vec<Event>, TalliiError> {
        // start the query
        let mut query = Self::get_event_query();

        // filter by the user
        query.push_str("where ep.user_id = $1 and ep.status = 'accepted");

        // execute the query and format the response
        let events = sqlx::query_as::<_, Event>(&query)
            .bind(params.user_id)
            .fetch_all(pool)
            .await?;

        Ok(events)
    }

    /// Gets all Events for me
    pub async fn get_me_many(
        pool: &PgPool,
        user: &AuthenticatedUser,
        params: &MeEventQueryParams,
    ) -> Result<Vec<Event>, TalliiError> {
        // start the query
        let mut query = Self::get_event_query();

        // filter by the user
        query.push_str(&format!("where ep.user_id = $1"));

        // add the optional clause for player status
        if let Some(player_status) = &params.player_status {
            query.push_str(&format!(" and ep.status = '{}'", player_status.to_string()));
        } else {
            query.push_str(" and ep.status = 'accepted")
        }

        // execute the query and format the response
        let events = sqlx::query_as::<_, Event>(&query)
            .bind(user.user_id)
            .fetch_all(pool)
            .await?;

        Ok(events)
    }
}
