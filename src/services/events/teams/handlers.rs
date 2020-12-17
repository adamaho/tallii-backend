use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::TalliiResponse;

use super::db::{EventsTeamsTable, EventTeamMembersTable};
use super::models::{NewTeam, TeamPlayerQueryParams, TeamQueryParams, UpdateTeamRequest};
use crate::services::events::members::db::EventMembersTable;
use crate::errors::TalliiError;
use crate::services::events::members::models::EventMember;

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
    user: AuthenticatedUser,
    event_id: web::Path<i32>,
    team: web::Json<NewTeam>,
) -> TalliiResponse {
    // check if the user is a member
    let is_member = EventMembersTable::exists(&pool, &event_id, &user.user_id).await?;

    // if not a member return forbidden
    if !is_member {
        return Err(TalliiError::FORBIDDEN.default());
    }

    // check to make sure all of the team members are members
    let mut event_members: Vec<Option<EventMember>> = Vec::new();

    // get each member
    for user_id in team.members.clone().into_iter() {
        event_members.push(EventMembersTable::get_member_by_user_id(&pool, &event_id, &user_id).await?);
    }

    // if not all members return bad request
    if event_members.iter().any(|member| member.is_none()) {
        return Err(TalliiError::BAD_REQUEST.message(String::from("Not all members are a part of this event.")));
    }

    // start the transaction
    let mut tx = pool.begin().await?;

    // create the team
    let new_team = EventsTeamsTable::create(&mut tx, &event_id, &team).await?;

    // create the team members
    EventTeamMembersTable::create_many(&mut tx, &new_team.team_id, &event_members).await?;

    // commit the transaction
    tx.commit().await?;

    // respond with created
    Ok(HttpResponse::Created().finish())
}

/// Updates a specific team
pub async fn update_team(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    team_id: web::Path<i32>,
    team: web::Json<UpdateTeamRequest>,
) -> TalliiResponse {
    EventsTeamsTable::update(&pool, &team_id, &team).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Gets all Teams and Members for an Event
pub async fn get_team_players(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Query<TeamPlayerQueryParams>,
) -> TalliiResponse {
    let players = EventTeamMembersTable::get_many(&pool, &params).await?;

    Ok(HttpResponse::Ok().json(players))
}
