use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::events::db::{EventParticipantRepository, EventRepository};
use crate::services::events::models::NewEvent;
use crate::services::TalliiResponse;

/// Creates a new Event
pub async fn create(
    pool: web::Data<PgPool>,
    new_event: web::Json<NewEvent>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    // start the transaction
    let mut tx = pool.begin().await?;

    // create new event in the transaction
    let created_event = EventRepository::create(&mut tx, &new_event, &user).await?;

    // create the participants in the transaction
    EventParticipantRepository::create_many(
        &mut tx,
        &created_event.event_id,
        &user.user_id,
        &new_event.participants,
    )
    .await?;

    // commit the transaction
    tx.commit().await?;

    // respond with json saying the event is created
    Ok(HttpResponse::Created().json("event created"))
}

// Gets all Events in Group
// pub async fn get_events(
//     pool: web::Data<PgPool>,
//     _user: AuthenticatedUser,
//     params: web::Query<EventParams>,
// ) -> TalliiResponse {
//     // TODO: validate user is apart of the group
//
//     let events = EventRepository::get_many_by_group_id(&pool, &params).await?;
//
//     Ok(HttpResponse::Ok().json(events))
// }
//
// // Gets all Teams and Members for an Event
// pub async fn get_event_teams(
//     pool: web::Data<PgPool>,
//     _user: AuthenticatedUser,
//     params: web::Query<EventTeamParams>,
// ) -> TalliiResponse {
//     // TODO: validate user is apart of the group
//
//     let teams = EventTeamRepository::get_many_by_event_id(&pool, &params).await?;
//
//     Ok(HttpResponse::Ok().json(teams))
// }
//
// // Gets all Teams and Members for an Event
// pub async fn get_event_team_members(
//     pool: web::Data<PgPool>,
//     _user: AuthenticatedUser,
//     params: web::Query<EventTeamMemberParams>,
// ) -> TalliiResponse {
//     // TODO: validate user is apart of the group
//
//     let members = EventTeamMemberRepository::get_many_by_event_id(&pool, &params).await?;
//
//     Ok(HttpResponse::Ok().json(members))
// }
//
// // Updates a team
// pub async fn update_team(
//     pool: web::Data<PgPool>,
//     _user: AuthenticatedUser,
//     event_team_id: web::Path<i32>,
//     team: web::Json<EditEventTeam>,
// ) -> TalliiResponse {
//     // TODO: validate user is apart of the group
//
//     EventTeamRepository::update(&pool, &event_team_id, &team).await?;
//
//     Ok(HttpResponse::Ok().finish())
// }
