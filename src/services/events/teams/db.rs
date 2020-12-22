use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;

use super::models::{NewTeam, Team};
use crate::services::events::members::models::EventMember;
use crate::services::events::teams::models::UpdateTeamRequest;
use crate::services::users::models::PublicUser;

pub struct EventsTeamsTable;

impl EventsTeamsTable {
    /// Creates an event team in the database
    pub async fn create(
        tx: &mut Transaction<'_, sqlx::Postgres>,
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
                    team_id,
                    event_id,
                    name,
                    score,
                    winner,
                    created_at
                from
                    events_teams
                where
                    event_id = $1
                order by
                    team_id asc
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
                    events_teams
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

    /// Delete a specific team
    pub async fn delete(pool: &PgPool, team_id: &i32) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                delete from
                    events_teams
                where
                    team_id = $1
            "#,
        )
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
        tx: &mut Transaction<'_, sqlx::Postgres>,
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
            query.push_str(&format!(
                "({}, {})",
                team_id,
                member.as_ref().unwrap().member_id
            ));

            // if we are appending values onto the query we need to separate them with commas
            if i < members.len() - 1 {
                query.push_str(",")
            }
        }

        // execute the query
        sqlx::query(&query).execute(tx).await?;

        Ok(())
    }

    /// Creates a team member for a team
    pub async fn create_one(
        pool: &PgPool,
        team_id: &i32,
        member: &EventMember,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                insert into
                    events_teams_members (team_id, member_id)
                values
                   ($1, $2)
            "#,
        )
        .bind(team_id)
        .bind(member.member_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Gets team members for a single event
    pub async fn get_many(pool: &PgPool, team_id: &i32) -> Result<Vec<PublicUser>, TalliiError> {
        let members = sqlx::query_as::<_, PublicUser>(
            r#"
                select
                    u.user_id,
                    u.avatar,
                    u.username,
                    u.taunt
                from
                    events_teams_members t
                left join
                    events_members em
                on
                    em.member_id = t.member_id
                left join
                    users u
                on
                    u.user_id = em.user_id
                where
                    t.team_id = $1
            "#,
        )
        .bind(team_id)
        .fetch_all(pool)
        .await?;

        Ok(members)
    }

    /// Deletes a team member from a team
    pub async fn delete(pool: &PgPool, team_id: &i32, user_id: &i32) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                delete from
                    events_teams_members etm
                using
                    events_members em
                where
                    etm.member_id = em.member_id
                and
                    em.user_id = $1
                and
                    etm.team_id = $2
            "#,
        )
        .bind(user_id)
        .bind(team_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
