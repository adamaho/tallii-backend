use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;

use super::models::{NewTeam, Team};
use crate::services::events::teams::models::{
    TeamPlayer, TeamPlayerQueryParams, UpdateTeamRequest,
};
use crate::services::events::members::models::EventMember;

pub struct EventsTeamsTable;

impl EventsTeamsTable {
    /// Creates an event team in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_id: &i32,
        team: &NewTeam,
    ) -> Result<Team, TalliiError> {
        // execute the query
        let created_team = sqlx::query_as::<_, Team>(
            r#"
                insert
                    into events_teams (event_id, name)
                values
                    ($1, $2)
                returning *
            "#,
        )
        .bind(event_id)
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
                    teams.team_id,
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

    /// Update a specific team
    pub async fn update(
        pool: &PgPool,
        team_id: &i32,
        team: &UpdateTeamRequest,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                update
                    teams
                set
                    name = $1,
                    score = $2,
                    winner = $3
                where
                    team_id = $4
            "#,
        )
        .bind(&team.name)
        .bind(&team.score)
        .bind(&team.winner)
        .bind(team_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}

pub struct EventTeamMembersTable;

impl EventTeamMembersTable {
    /// Creates many event team players
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        team_id: &i32,
        members: &Vec<Option<EventMember>>,
    ) -> Result<(), TalliiError> {
        // check if each member is a part of the event

        // init the query
        let mut query = String::from(
            r#"
                insert
                    into events_teams_members (team_id, member_id)
                values
            "#,
        );

        // create the queries for each of the new participants
        for (i, member) in members.into_iter().enumerate() {
            query.push_str(&format!("({}, {})", team_id, member.as_ref().unwrap().member_id));

            // if we are appending values onto the query we need to separate them with commas
            if i < members.len() - 1 {
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
        params: &TeamPlayerQueryParams,
    ) -> Result<Vec<TeamPlayer>, TalliiError> {
        let players = sqlx::query_as::<_, TeamPlayer>(
            r#"
                select
                    teams_players.team_id,
                    teams_players.player_id,
                    teams_players.created_at
                from
                    teams_players
                left join
                    teams t
                on
                    teams_players.team_id = t.team_id
                where
                    t.event_id = $1
            "#,
        )
        .bind(params.event_id)
        .fetch_all(pool)
        .await?;

        Ok(players)
    }
}
