use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::{SuccessResponse, TalliiResponse};

use super::db::{EventTeamMembersTable, EventsTeamsTable};
use super::models::{NewTeam, UpdateTeamRequest};
use crate::errors::TalliiError;
use crate::services::events::members::db::EventMembersTable;
use crate::services::events::members::models::EventMember;

/// Gets all Teams for an event
pub async fn get_teams(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    let teams = EventsTeamsTable::get_many(&pool, &event_id).await?;

    Ok(HttpResponse::Ok().json(teams))
}

/// Gets a single Team for an event
pub async fn get_team(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Path<(i32, i32)>,
) -> TalliiResponse {
    let (event_id, team_id) = params.into_inner();

    let teams = EventsTeamsTable::get_one(&pool, &event_id, &team_id).await?;

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
        event_members
            .push(EventMembersTable::get_member_by_user_id(&pool, &event_id, &user_id).await?);
    }

    // if not all members return bad request
    if event_members.iter().any(|member| member.is_none()) {
        return Err(TalliiError::BAD_REQUEST
            .message(String::from("Not all members are a part of this event.")));
    }

    // start the transaction
    let mut tx = pool.begin().await?;

    // create the team
    let new_team = EventsTeamsTable::create(&mut tx, &event_id, &team).await?;

    if team.members.len() > 0 {
        // create the team members
        EventTeamMembersTable::create_many(&mut tx, &new_team.team_id, &event_members).await?;
    }

    // commit the transaction
    tx.commit().await?;

    // respond with created
    Ok(HttpResponse::Ok().json(SuccessResponse {
        code: String::from("CREATED_EVENT_TEAM"),
        message: String::from("The provided team was created."),
    }))
}

/// Updates a specific team
pub async fn update_team(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path_params: web::Path<(i32, i32)>,
    team: web::Json<UpdateTeamRequest>,
) -> TalliiResponse {
    let (event_id, team_id) = path_params.into_inner();

    // check if the user is a member
    let is_member = EventMembersTable::exists(&pool, &event_id, &user.user_id).await?;

    // if not a member return forbidden
    if !is_member {
        return Err(TalliiError::FORBIDDEN.default());
    }

    EventsTeamsTable::update(&pool, &team_id, &team).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        code: String::from("UPDATED_EVENT_TEAM"),
        message: String::from("The provided team was updated."),
    }))
}

/// deletes a specific team
pub async fn delete_team(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path_params: web::Path<(i32, i32)>,
) -> TalliiResponse {
    let (event_id, team_id) = path_params.into_inner();

    // check if the user is a member
    let is_member = EventMembersTable::exists(&pool, &event_id, &user.user_id).await?;

    // if not a member return forbidden
    if !is_member {
        return Err(TalliiError::FORBIDDEN.default());
    }

    EventsTeamsTable::delete(&pool, &team_id).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        code: String::from("DELETED_EVENT_TEAM"),
        message: String::from("The provided team was deleted."),
    }))
}

/// Gets all Teams and Members for an Event
pub async fn get_team_members(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    path_params: web::Path<(i32, i32)>,
) -> TalliiResponse {
    let (_event_id, team_id) = path_params.into_inner();

    let members = EventTeamMembersTable::get_many(&pool, &team_id).await?;

    Ok(HttpResponse::Ok().json(members))
}

/// Removes a member from a team
pub async fn delete_team_member(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path_params: web::Path<(i32, i32, i32)>,
) -> TalliiResponse {
    let (event_id, team_id, user_id) = path_params.into_inner();

    // check if the user is a member
    let is_member = EventMembersTable::exists(&pool, &event_id, &user.user_id).await?;

    // if not a member return forbidden
    if !is_member {
        return Err(TalliiError::FORBIDDEN.default());
    }

    // delete the team member
    EventTeamMembersTable::delete(&pool, &team_id, &user_id).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        code: String::from("DELETED_EVENT_TEAM_MEMBER"),
        message: String::from("The provided team member was deleted."),
    }))
}


/// Changes team of the provided user
pub async fn change_team(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path_params: web::Path<(i32, i32, i32)>,
) -> TalliiResponse {
    let (event_id, team_id, user_id) = path_params.into_inner();

    // check if the user is a member
    let is_requester_member = EventMembersTable::exists(&pool, &event_id, &user.user_id).await?;

    // if not a member return forbidden
    if !is_requester_member {
        return Err(TalliiError::FORBIDDEN.default());
    }

    // check if the user is a member
    let user_member = EventMembersTable::get_member_by_user_id(&pool, &event_id, &user_id).await?;

    // if not a member return forbidden
    if user_member.is_none() {
        return Err(TalliiError::BAD_REQUEST.message(String::from(
            "The provided user is not a member of this event.",
        )));
    }

    // start the transaction
    let mut tx = pool.begin().await?;

    // delete the team member
    EventTeamMembersTable::delete_by_event_id(&mut tx, &event_id, &user_id).await?;

    // add the team member to the new table
    EventTeamMembersTable::create_one_tx(&mut tx, &team_id, &user_member.unwrap()).await?;

    // commit the transaction
    tx.commit().await?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        code: String::from("CHANGED_USER_EVENT_TEAM"),
        message: String::from("The provided member changed teams."),
    }))
}

/// Adds a member to a team
pub async fn add_team_member(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path_params: web::Path<(i32, i32, i32)>,
) -> TalliiResponse {
    let (event_id, team_id, user_id) = path_params.into_inner();

    // check if the user is a member
    let is_requester_member = EventMembersTable::exists(&pool, &event_id, &user.user_id).await?;

    // if not a member return forbidden
    if !is_requester_member {
        return Err(TalliiError::FORBIDDEN.default());
    }

    // check if the user is a member
    let user_member = EventMembersTable::get_member_by_user_id(&pool, &event_id, &user_id).await?;

    // if not a member return forbidden
    if user_member.is_none() {
        return Err(TalliiError::BAD_REQUEST.message(String::from(
            "The provided user is not a member of this event.",
        )));
    }

    // add the team member
    EventTeamMembersTable::create_one(&pool, &team_id, &user_member.unwrap()).await?;

    Ok(HttpResponse::Ok().json(SuccessResponse {
        code: String::from("ADDED_EVENT_TEAM_MEMBER"),
        message: String::from("The provided team member was added."),
    }))
}
