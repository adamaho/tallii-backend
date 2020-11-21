use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::events::models::{Event, EventTeam, NewEvent, NewEventTeam, EventQueryParams};

pub struct EventRepository;

impl EventRepository {
    /// Creates an event in the database
    pub async fn create(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        new_event: &NewEvent,
        user: &AuthenticatedUser,
    ) -> Result<Event, TalliiError> {
        let event = sqlx::query_as::<_, Event>(
            r#"
                insert into events (name, description, creator_user_id)
                values ($1, $2, $3)
                returning *
            "#,
        )
        .bind(&new_event.name)
        .bind(&new_event.description)
        .bind(&user.user_id)
        .fetch_one(tx)
        .await?;

        Ok(event)
    }

    /// Gets all Events for user
    pub async fn get_many(
        pool: &PgPool,
        user: &AuthenticatedUser,
        params: &EventQueryParams,
    ) -> Result<Vec<Event>, TalliiError> {

        // start the query
        let mut query = String::from(
            r#"
                select
                    events.event_id, events.name, events.description, events.creator_user_id, events.created_at
                from
                    events
                left join
                    events_participants ep
                on
                    events.event_id = ep.event_id
            "#
        );

        // filter by the user
        query.push_str(&format!("where ep.user_id = {}", user.user_id));

        // add the optional clause for participant status
        if let Some(participant_status) = &params.participant_status {
            query.push_str(&format!(" and event_id = {}", participant_status));
        }

        // execute the query
        let events = sqlx::query_as::<_, Event>(&query).fetch_all(pool).await?;

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
        let mut query = String::from("insert into events_participants (event_id, user_id, status) values");

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
}

// pub struct EventTeamRepository;
//
// impl EventTeamRepository {
//     /// Creates an events_teams in the database
//     pub async fn create(
//         tx: &mut Transaction<PoolConnection<PgConnection>>,
//         event_id: &i32,
//         team: &NewEventTeam,
//     ) -> Result<EventTeam, TalliiError> {
//         // execute the query
//         let created_team = sqlx::query_as::<_, EventTeam>(
//             "insert into events_teams (event_id, name) values ($1, $2) returning *",
//         )
//         .bind(&event_id)
//         .bind(&team.name)
//         .fetch_one(tx)
//         .await?;
//
//         Ok(created_team)
//     }
//
//     // Gets all Event Teams by event_id
//     pub async fn get_many_by_event_id(
//         pool: &PgPool,
//         params: &EventTeamParams,
//     ) -> Result<Vec<EventTeam>, TalliiError> {
//         // start the query
//         let mut query = String::from("select * from events_teams");
//
//         // filter by event_id if available
//         if let Some(event_id) = params.event_id {
//             query.push_str(&format!(" where event_id = {}", event_id));
//         }
//
//         let teams = sqlx::query_as::<_, EventTeam>(&query)
//             .fetch_all(pool)
//             .await?;
//
//         Ok(teams)
//     }
//
//     // Updates a team
//     pub async fn update(
//         pool: &PgPool,
//         event_team_id: &i32,
//         team: &EditEventTeam,
//     ) -> Result<(), TalliiError> {
//         sqlx::query_as::<_, EventTeam>(
//             "update events_teams set name = $1, score = $2, winner = $3 where event_team_id = $4 returning *"
//         )
//             .bind(&team.name)
//             .bind(&team.score)
//             .bind(&team.winner)
//             .bind(event_team_id)
//             .fetch_one(pool)
//             .await?;
//
//         Ok(())
//     }
// }
//
// pub struct EventTeamMemberRepository;
//
// impl EventTeamMemberRepository {
//     // Creates an events_teams_members in the database
//     pub async fn create_many(
//         tx: &mut Transaction<PoolConnection<PgConnection>>,
//         event_team_id: &i32,
//         new_members: &Vec<NewEventTeamMember>,
//     ) -> Result<(), TalliiError> {
//         let mut query =
//             String::from("insert into events_teams_members (event_team_id, user_id) values");
//
//         // create the queries for each of the new members and add them to the query string
//         for (i, member) in new_members.iter().enumerate() {
//             query.push_str(&format!("({}, {})", event_team_id, member.user_id));
//
//             // if we are appending values onto the query we need to separate them with commas
//             if i < new_members.len() - 1 {
//                 query.push_str(",")
//             }
//         }
//
//         // execute the query
//         sqlx::query(&query).execute(tx).await?;
//
//         Ok(())
//     }
//
//     // Gets all Event Team Members by event_id
//     pub async fn get_many_by_event_id(
//         pool: &PgPool,
//         params: &EventTeamMemberParams,
//     ) -> Result<Vec<EventTeamMember>, TalliiError> {
//         // start the query
//         let mut query = String::from(
//             r#"
//             select
//                 event_team_member_id, members.event_team_id, users.user_id, username, avatar, taunt, members.created_at
//             from
//                 events_teams as teams
//             inner join
//                 events_teams_members as members
//             on
//                 teams.event_team_id = members.event_team_id
//             inner join
//                 users
//             on
//                 members.user_id = users.user_id
//         "#,
//         );
//
//         // filter by event_id if available
//         if let Some(event_id) = &params.event_id {
//             query.push_str(&format!("where event_id = {}", event_id))
//         }
//
//         // execute the query
//         let members = sqlx::query_as::<_, EventTeamMember>(&query)
//             .fetch_all(pool)
//             .await?;
//
//         Ok(members)
//     }
// }
