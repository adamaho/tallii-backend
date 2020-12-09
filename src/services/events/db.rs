use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;

use crate::services::events::models::{
    Event,
    EventCreator,
    EventQueryParams,
    EventResponsePayload,
    EventRow,
    NewEvent
};

pub struct EventsTable;

impl EventsTable {
    // initial part of the query for getting events
    fn get_event_query() -> String {
        String::from(
            r#"
                select
                    events.event_id, events.name, events.description, events.creator_user_id, events.created_at, u.user_id, u.username, u.avatar
                from
                    events
                left join
                    events_participants ep
                on
                    events.event_id = ep.event_id
                left join
                    users u
                on
                    events.creator_user_id = u.user_id
            "#,
        )
    }

    /// builds the response for getting an event
    fn build_get_event_response(e: EventRow) -> EventResponsePayload {
        EventResponsePayload {
            event_id: e.event_id,
            name: e.name,
            description: e.description,
            creator: EventCreator {
                user_id: e.user_id,
                username: e.username,
                avatar: e.avatar,
            },
            created_at: e.created_at,
        }
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
    pub async fn get_one(
        pool: &PgPool,
        event_id: &i32,
    ) -> Result<EventResponsePayload, TalliiError> {
        // get the initial query
        let mut query = Self::get_event_query();

        // push the predicate
        query.push_str("where events.event_id = $1");

        // execute the query
        let event_row = sqlx::query_as::<_, EventRow>(&query)
            .bind(event_id)
            .fetch_one(pool)
            .await?;

        // format the response
        let event_response = Self::build_get_event_response(event_row);

        // return the result
        Ok(event_response)
    }

    /// Gets all Events for user
    pub async fn get_many(
        pool: &PgPool,
        _user: &AuthenticatedUser,
        params: &EventQueryParams,
    ) -> Result<Vec<EventResponsePayload>, TalliiError> {
        // start the query
        let mut query = Self::get_event_query();

        // filter by the user
        query.push_str(&format!("where ep.user_id = {}", params.user_id));

        // add the optional clause for player status
        if let Some(player_status) = &params.player_status {
            query.push_str(&format!(" and ep.status = '{}'", player_status));
        }

        // execute the query and format the response
        let events = sqlx::query_as::<_, EventRow>(&query)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|e| {
                return Self::build_get_event_response(e);
            })
            .collect();

        Ok(events)
    }
}
