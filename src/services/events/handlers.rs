use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::events::db::{
    EventRepository, EventTeamMemberRepository, EventTeamRepository,
};
use crate::services::events::models::{NewEventRequest, EventTeamParams, EventParams, EventTeamMemberParams};
use crate::services::TalliiResponse;

/// Creates a new Event
pub async fn create(
    pool: web::Data<PgPool>,
    new_event: web::Json<NewEventRequest>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    // start the transaction
    let mut tx = pool.begin().await?;

    // TODO: validate that all team members are a part of the group

    // create new event in the transaction
    let created_event = EventRepository::create(&mut tx, &new_event.event, &user).await?;

    // loop through the teams
    for team_request in new_event.teams.iter() {
        // create new event teams
        let created_team = EventTeamRepository::create(
            &mut tx,
            &created_event.event_id,
            &team_request.team,
        )
        .await?;

        // create the associated members of the team
        EventTeamMemberRepository::create_many(&mut tx, &created_team.event_team_id, &team_request.members).await?;
    }

    tx.commit().await?;

    Ok(HttpResponse::Created().finish())
}

/// Gets all Events in Group
pub async fn get_events(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Query<EventParams>
) -> TalliiResponse {
    // TODO: validate user is apart of the group

    let events = EventRepository::get_many_by_group_id(&pool, &params).await?;

    Ok(HttpResponse::Ok().json(events))
}

/// Gets all Teams and Members for an Event
pub async fn get_event_teams(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Query<EventTeamParams>
) -> TalliiResponse {
    // TODO: validate user is apart of the group

    let teams = EventTeamRepository::get_many_by_event_id(&pool, &params).await?;

    Ok(HttpResponse::Ok().json(teams))
}

/// Gets all Teams and Members for an Event
pub async fn get_event_members(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Query<EventTeamMemberParams>
) -> TalliiResponse {
    // TODO: validate user is apart of the group

    let members = EventTeamMemberRepository::get_many_by_event_id(&pool, &params).await?;

    Ok(HttpResponse::Ok().json(members))
}
