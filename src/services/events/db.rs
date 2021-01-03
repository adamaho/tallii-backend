use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;

use crate::services::events::models::{
    CreateEventRequest, Event, EventResponse, EventRow, UpdateEventRequest,
};
use crate::services::users::models::PublicUser;

pub struct EventsTable;

impl EventsTable {
    /// Creates an event in the database
    pub async fn create(
        tx: &mut Transaction<'_, sqlx::Postgres>,
        new_event: &CreateEventRequest,
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

    /// Gets events that a specific user id is a part of
    pub async fn get_events_for_user_id(
        pool: &PgPool,
        user_id: &i32,
        state: &str,
    ) -> Result<Vec<EventResponse>, TalliiError> {
        let events = sqlx::query_as::<_, EventRow>(
            r#"
                select
                    events.event_id,
                    events.name,
                    events.description,
                    u.user_id,
                    u.username,
                    u.bio,
                    u.emoji,
                    u.bg_color,
                    events.created_at
                from
                    events
                left join
                    events_members em
                on
                    events.event_id = em.event_id
                left join
                    users u
                on
                    events.creator_user_id = u.user_id
                where
                    em.user_id = $1 and state = $2
            "#,
        )
        .bind(user_id)
        .bind(state)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|event| EventResponse {
            event_id: event.event_id,
            name: event.name,
            description: event.description,
            creator: PublicUser {
                user_id: event.user_id,
                emoji: event.emoji,
                bg_color: event.bg_color,
                username: event.username,
                bio: event.bio,
            },
            created_at: event.created_at,
        })
        .collect();

        Ok(events)
    }

    /// Gets events that a specific user id is a part of
    pub async fn get_event_by_id(
        pool: &PgPool,
        event_id: &i32,
    ) -> Result<EventResponse, TalliiError> {
        let event = sqlx::query_as::<_, EventRow>(
            r#"
                select
                    events.event_id,
                    events.name,
                    events.description,
                    u.user_id,
                    u.username,
                    u.bio,
                    u.emoji,
                    u.bg_color,
                    events.created_at
                from
                    events
                left join
                    events_members em
                on
                    events.event_id = em.event_id
                left join
                    users u
                on
                    events.creator_user_id = u.user_id
                where
                    events.event_id = $1
            "#,
        )
        .bind(event_id)
        .fetch_optional(pool)
        .await?;

        match event {
            Some(event) => {
                let event_to_return = EventResponse {
                    event_id: event.event_id,
                    name: event.name,
                    description: event.description,
                    creator: PublicUser {
                        user_id: event.user_id,
                        emoji: event.emoji,
                        bg_color: event.bg_color,
                        username: event.username,
                        bio: event.bio,
                    },
                    created_at: event.created_at,
                };

                Ok(event_to_return)
            }
            None => Err(TalliiError::NOT_FOUND.default()),
        }
    }

    /// Updates an event with the provided event_id
    pub async fn update_event_by_id(
        pool: &PgPool,
        event_id: &i32,
        update_event_request: &UpdateEventRequest,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                update
                    events
                set
                    name = $1,
                    description = $2
                where
                    event_id = $3
            "#,
        )
        .bind(&update_event_request.name)
        .bind(&update_event_request.description)
        .bind(event_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Updates an event with the provided event_id
    pub async fn delete_event_by_id(pool: &PgPool, event_id: &i32) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                delete from
                    events
                where
                    event_id = $1
            "#,
        )
        .bind(event_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
