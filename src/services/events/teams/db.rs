use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;

use super::models::{Team, NewTeam};
use crate::services::events::teams::models::TeamPlayer;

pub struct EventsTeamsTable;

impl EventsTeamsTable {
    /// Creates an event team in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        team: &NewTeam,
    ) -> Result<Team, TalliiError> {
        // execute the query
        let created_team = sqlx::query_as::<_, Team>(
            r#"
                insert
                    into teams (event_id, name)
                values
                    ($1, $2)
                returning *
            "#,
        )
        .bind(&team.event_id)
        .bind(&team.name)
        .fetch_one(tx)
        .await?;

        Ok(created_team)
    }

    // Gets all teams for a single event
    pub async fn get_many(pool: &PgPool, event_id: &i32) -> Result<Vec<Team>, TalliiError> {
        let teams = sqlx::query_as::<_, Team>(
            r#"
                select
                    teams.event_team_id,
                    teams.event_id,
                    teams.name,
                    teams.score,
                    teams.winner,
                    teams.created_at
                from
                    teams
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

pub struct TeamsPlayersTable;

impl TeamsPlayersTable {
    /// Creates many event team players
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        team_id: &i32,
        players: &Vec<i32>,
    ) -> Result<(), TalliiError> {
        // init the query
        let mut query = String::from(
            r#"
                insert
                    into teams_players (team_id, player_id)
                values
            "#,
        );

        // create the queries for each of the new participants
        for (i, player_id) in players.into_iter().enumerate() {
            query.push_str(&format!("({}, {})", team_id, player_id));

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
        team_id: &i32,
    ) -> Result<Vec<TeamPlayer>, TalliiError> {
        let players = sqlx::query_as::<_, TeamPlayer>(
            r#"
                select
                    teams.team_id,
                    etp.player_id,
                    etp.created_at
                from
                    teams
                left join
                    teams_players etp
                on
                    teams.team_id = etp.team_id
                where
                    team_id = $1;
            "#,
        )
        .bind(team_id)
        .fetch_all(pool)
        .await?;

        Ok(players)
    }
}
