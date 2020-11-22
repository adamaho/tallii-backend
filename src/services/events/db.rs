use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::events::models::{
    Event, EventCreator, EventParticipant, EventParticipantRequest, EventParticipantRow,
    EventQueryParams, EventResponsePayload, EventRow, EventTeam, EventTeamRow, NewEvent,
    NewEventTeam, EventTeamResponse
};

pub struct EventRepository;

impl EventRepository {
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
        user: &AuthenticatedUser,
        params: &EventQueryParams,
    ) -> Result<Vec<EventResponsePayload>, TalliiError> {
        // start the query
        let mut query = Self::get_event_query();

        // filter by the user
        query.push_str(&format!("where ep.user_id = {}", user.user_id));

        // add the optional clause for participant status
        if let Some(participant_status) = &params.participant_status {
            query.push_str(&format!(" and ep.status = '{}'", participant_status));
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

pub struct EventParticipantRepository;

impl EventParticipantRepository {
    /// Creates many event participants in the database
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_id: &i32,
        user_id: &i32,
        participants: &Vec<i32>,
    ) -> Result<(), TalliiError> {
        // init the query
        let mut query =
            String::from("insert into events_participants (event_id, user_id, status) values");

        // add the current user to the participants
        query.push_str(&format!("({}, {}, 'accepted'),", event_id, user_id));

        // create the queries for each of the new members and add them to the query string
        for (i, user_id) in participants.iter().enumerate() {
            query.push_str(&format!("({}, {}, 'pending')", event_id, user_id));

            // if we are appending values onto the query we need to separate them with commas
            if i < participants.len() - 1 {
                query.push_str(",")
            }
        }

        // execute the query
        sqlx::query(&query).execute(tx).await?;

        Ok(())
    }

    /// Gets all Participants for a single event
    pub async fn get_many(
        pool: &PgPool,
        event_id: &i32,
    ) -> Result<Vec<EventParticipantRow>, TalliiError> {
        let participants = sqlx::query_as::<_, EventParticipantRow>(
            r#"
                select
                    events_participants.event_participant_id,
                    events_participants.event_id,
                    u.user_id,
                    u.username,
                    u.avatar,
                    u.taunt,
                    events_participants.status,
                    events_participants.created_at
                from
                    events_participants
                left join
                    users u
                on
                    events_participants.user_id = u.user_id
                where
                    event_id = $1;
            "#,
        )
        .bind(event_id)
        .fetch_all(pool)
        .await?;

        Ok(participants)
    }

    /// Updates a single participant
    pub async fn update(
        pool: &PgPool,
        event_participant_id: &i32,
        participant: &EventParticipantRequest,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                update
                    events_participants
                set
                    user_id = $1,
                    status = $2
                where
                    event_participant_id = $3
            "#,
        )
        .bind(&participant.user_id)
        .bind(&participant.status)
        .bind(event_participant_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub struct EventTeamRepository;

impl EventTeamRepository {
    /// Creates an event team in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_id: &i32,
        team: &NewEventTeam,
    ) -> Result<EventTeam, TalliiError> {
        // execute the query
        let created_team = sqlx::query_as::<_, EventTeam>(
            r#"
                insert
                    into events_teams (event_id, name)
                values
                    ($1, $2)
                returning *
            "#,
        )
        .bind(&event_id)
        .bind(&team.name)
        .fetch_one(tx)
        .await?;

        Ok(created_team)
    }

    // Gets all teams for a single event
    pub async fn get_many(pool: &PgPool, event_id: &i32) -> Result<(), TalliiError> {
        let mut teams: Vec<EventTeamResponse> = Vec::new();
        sqlx::query_as::<_, EventTeamRow>(
            r#"
                select
                    events_teams.event_team_id,
                    events_teams.event_id,
                    events_teams.name,
                    events_teams.score,
                    events_teams.winner,
                    events_teams.created_at,
                    etp.event_participant_id
                from
                    events_teams
                left join
                    events_teams_participants etp
                on
                    events_teams.event_team_id = etp.event_team_id
                where
                    event_id = $1;
            "#,
        )
        .bind(event_id)
        .fetch_all(pool)
        .await?
        .into_iter().for_each(|row| {
            println!("{:?}", row);
        });

        Ok(())
    }
}

pub struct EventTeamParticipantsRepository;

impl EventTeamParticipantsRepository {
    /// Creates many event team participants in the database
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_team_id: &i32,
        participants: &Vec<i32>,
    ) -> Result<(), TalliiError> {
        // init the query
        let mut query = String::from(
            r#"
                    insert
                        into events_teams_participants (event_team_id, event_participant_id)
                    values
                "#,
        );

        // create the queries for each of the new participants
        for (i, event_participant_id) in participants.into_iter().enumerate() {
            query.push_str(&format!("({}, {})", event_team_id, event_participant_id));

            // if we are appending values onto the query we need to separate them with commas
            if i < participants.len() - 1 {
                query.push_str(",")
            }
        }

        // execute the query
        sqlx::query(&query).execute(tx).await?;

        Ok(())
    }
}
