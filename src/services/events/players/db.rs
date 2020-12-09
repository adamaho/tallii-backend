use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;
use crate::services::events::models::PlayerStatus;

use super::models::{EventPlayerRequest, EventPlayerRow};

pub struct EventsPlayersTable;

impl EventsPlayersTable {
    /// Creates many event players in the database
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_id: &i32,
        user_id: &i32,
        players: &Vec<i32>,
    ) -> Result<(), TalliiError> {
        // init the query
        let mut query =
            String::from("insert into events_players (event_id, user_id, status) values");

        // add the current user to the players
        query.push_str(&format!("({}, {}, 'accepted'),", event_id, user_id));

        // create the queries for each of the new players and add them to the query string
        for (i, user_id) in players.iter().enumerate() {
            query.push_str(&format!("({}, {}, 'pending')", event_id, user_id));

            // if we are appending values onto the query we need to separate them with commas
            if i < players.len() - 1 {
                query.push_str(",")
            }
        }

        // execute the query
        sqlx::query(&query).execute(tx).await?;

        Ok(())
    }

    /// Gets all Players for a single event
    pub async fn get_many(
        pool: &PgPool,
        event_id: &i32,
    ) -> Result<Vec<EventPlayerRow>, TalliiError> {
        let players = sqlx::query_as::<_, EventPlayerRow>(
            r#"
                select
                    events_players.event_player_id,
                    events_players.event_id,
                    u.user_id,
                    u.username,
                    u.avatar,
                    u.taunt,
                    events_players.status,
                    events_players.created_at
                from
                    events_players
                left join
                    users u
                on
                    events_players.user_id = u.user_id
                where
                    event_id = $1;
            "#,
        )
        .bind(event_id)
        .fetch_all(pool)
        .await?;

        Ok(players)
    }

    /// Updates a single player
    pub async fn update(
        pool: &PgPool,
        event_player_id: &i32,
        player: &EventPlayerRequest,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                update
                    events_players
                set
                    user_id = $1,
                    status = $2
                where
                    event_player_id = $3
            "#,
        )
        .bind(&player.user_id)
        .bind(&player.status.to_string())
        .bind(event_player_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
