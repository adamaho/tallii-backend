use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::TalliiResponse;

use super::db::{TeamsPlayersTable, EventsTeamsTable};
use super::models::{NewTeam, TeamQueryParams, TeamPlayerQueryParams};

/// Gets all Teams and Members for an Event
pub async fn get_teams(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Query<TeamQueryParams>,
) -> TalliiResponse {
    let teams = EventsTeamsTable::get_many(&pool, &params.event_id).await?;

    Ok(HttpResponse::Ok().json(teams))
}

/// Creates an event team
pub async fn create_team(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    team: web::Json<NewTeam>,
) -> TalliiResponse {
    // start the transaction
    let mut tx = pool.begin().await?;

    // create the team
    let new_team = EventsTeamsTable::create(&mut tx, &team).await?;

    // create the team players
    TeamsPlayersTable::create_many(&mut tx, &new_team.team_id, &team.players).await?;

    // commit the transaction
    tx.commit().await?;

    // respond with created
    Ok(HttpResponse::Created().finish())
}

/// Gets all Teams and Members for an Event
pub async fn get_team_players(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Query<TeamPlayerQueryParams>,
) -> TalliiResponse {
    let players = TeamsPlayersTable::get_many(&pool, &params).await?;

    Ok(HttpResponse::Ok().json(players))
}
