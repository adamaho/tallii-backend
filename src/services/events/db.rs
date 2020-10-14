use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::PgPool;
use sqlx::Transaction;

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::events::models::{Event, NewEvent, NewEventTeamRequest};

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


pub struct EventTeamRepository;

impl EventTeamRepository {
    /// Creates an event_team in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        new_event_teams: &Vec<NewEventTeamRequest>
    ) -> Result<(), TalliiError> {
        let mut query = String::from("insert into event_teams (event_id, name) values");

        // create the queries for each of the new teams and add them to the query string
        for (i, event_team) in new_event_teams.iter().enumerate() {
            query.push_str(&format!(
                "({}, {})",
                event_team.team.event_id, event_team.team.name
            ));

            // if we are appending values onto the query we need to separate them with commas
            if i < new_event_teams.len() - 1 {
                query.push_str(",")
            }
        }

        // execute the query
        sqlx::query(&query)
            .execute(tx)
            .await?;

        Ok(())
    }
}