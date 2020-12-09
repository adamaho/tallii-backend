use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;

use super::models::{EventTeam, NewEventTeam};
use crate::services::events::teams::models::EventTeamPlayer;

pub struct EventsTeamsTable;

impl EventsTeamsTable {
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
    pub async fn get_many(pool: &PgPool, event_id: &i32) -> Result<Vec<EventTeam>, TalliiError> {
        let teams = sqlx::query_as::<_, EventTeam>(
            r#"
                select
                    events_teams.event_team_id,
                    events_teams.event_id,
                    events_teams.name,
                    events_teams.score,
                    events_teams.winner,
                    events_teams.created_at
                from
                    events_teams
                where
                    event_id = $1;
            "#,
        )
            .bind(event_id)
            .fetch_all(pool)
            .await?;

        Ok(teams)
    }
}


pub struct EventsTeamsPlayersTable;

impl EventsTeamsPlayersTable {

    /// Creates many event team players
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_team_id: &i32,
        players: &Vec<i32>,
    ) -> Result<(), TalliiError> {
        // init the query
        let mut query = String::from(
            r#"
                    insert
                        into events_teams_players (event_team_id, event_player_id)
                    values
                "#,
        );

        // create the queries for each of the new participants
        for (i, event_player_id) in players.into_iter().enumerate() {
            query.push_str(&format!("({}, {})", event_team_id, event_player_id));

            // if we are appending values onto the query we need to separate them with commas
            if i < players.len() - 1 {
                query.push_str(",")
            }
        }

        // execute the query
        sqlx::query(&query).execute(tx).await?;

        Ok(())
    }

    // Gets team players for a single event
    pub async fn get_many(
        pool: &PgPool,
        event_id: &i32,
    ) -> Result<Vec<EventTeamPlayer>, TalliiError> {
        let players = sqlx::query_as::<_, EventTeamPlayer>(
            r#"
                select
                    events_teams.event_team_id,
                    etp.event_player_id,
                    etp.created_at
                from
                    events_teams
                left join
                    events_teams_players etp
                on
                    events_teams.event_team_id = etp.event_team_id
                where
                    event_id = $1;
            "#,
        )
            .bind(event_id)
            .fetch_all(pool)
            .await?;

        Ok(players)
    }
}