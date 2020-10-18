use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{Transaction, PgPool};

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::events::models::{
    Event, EventTeam, NewEvent, NewEventTeam, NewEventTeamMember,
};

pub struct EventRepository;

impl EventRepository {
    /// Creates an event in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        new_event: &NewEvent,
        group_id: &i32,
        user: &AuthenticatedUser,
    ) -> Result<Event, TalliiError> {
        let event = sqlx::query_as::<_, Event>(
            r#"
                insert into events (group_id, name, description, event_type, creator_user_id)
                values ($1, $2, $3, $4, $5)
                returning *
            "#,
        )
        .bind(group_id)
        .bind(&new_event.name)
        .bind(&new_event.description)
        .bind(&new_event.event_type)
        .bind(&user.user_id)
        .fetch_one(tx)
        .await?;

        Ok(event)
    }

    /// Gets all Events by group_id
    pub async fn get_many_by_group_id(
        pool: &PgPool,
        group_id: &i32
    ) -> Result<Vec<Event>, TalliiError> {
        let events = sqlx::query_as::<_, Event>(
            r#"
                select * from events
                where group_id = $1
            "#,
        )
            .bind(group_id)
            .fetch_all(pool)
            .await?;

        Ok(events)
    }
}

pub struct EventTeamRepository;

impl EventTeamRepository {
    /// Creates an events_teams in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_id: &i32,
        team: &NewEventTeam,
    ) -> Result<EventTeam, TalliiError> {
        let created_team = sqlx::query_as::<_, EventTeam>(
            "insert into events_teams (event_id, name) values ($1, $2) returning *",
        )
        .bind(&event_id)
        .bind(&team.name)
        .fetch_one(tx)
        .await?;

        Ok(created_team)
    }
}

pub struct EventTeamMemberRepository;

impl EventTeamMemberRepository {
    /// Creates an events_teams_members in the database
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_team_id: &i32,
        new_members: &Vec<NewEventTeamMember>,
    ) -> Result<(), TalliiError> {
        let mut query =
            String::from("insert into events_teams_members (event_team_id, user_id) values");

        // create the queries for each of the new members and add them to the query string
        for (i, member) in new_members.iter().enumerate() {
            query.push_str(&format!("({}, {})", event_team_id, member.user_id));

            // if we are appending values onto the query we need to separate them with commas
            if i < new_members.len() - 1 {
                query.push_str(",")
            }
        }

        // execute the query
        sqlx::query(&query).execute(tx).await?;

        Ok(())
    }
}
