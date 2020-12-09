use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::TalliiResponse;
use crate::services::auth::AuthenticatedUser;

use super::db::{EventsTeamsTable, EventsTeamsPlayersTable};
use super::models::NewEventTeam;

/// Gets all Teams and Members for an Event
pub async fn get_event_teams(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    let teams = EventsTeamsTable::get_many(&pool, &event_id).await?;

    Ok(HttpResponse::Ok().json(teams))
}

/// Creates an event team
pub async fn create_event_team(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
    team: web::Json<NewEventTeam>,
) -> TalliiResponse {
    // start the transaction
    let mut tx = pool.begin().await?;

    // create the team
    let new_team = EventsTeamsTable::create(&mut tx, &event_id, &team).await?;

    // create the team players
    EventsTeamsPlayersTable::create_many(
        &mut tx,
        &new_team.event_team_id,
        &team.players,
    )
        .await?;

    // commit the transaction
    tx.commit().await?;

    // respond with created
    Ok(HttpResponse::Created().json("Team Created."))
}

/// Gets all Teams and Members for an Event
pub async fn get_event_team_players(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    let players = EventsTeamsPlayersTable::get_many(&pool, &event_id).await?;

    Ok(HttpResponse::Ok().json(players))
}