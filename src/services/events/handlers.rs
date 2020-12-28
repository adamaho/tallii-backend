use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;

use super::db::EventsTable;

use super::models::{CreateEventRequest, CreatedEventResponse};

use super::members::db::EventMembersTable;

use crate::errors::TalliiError;
use crate::services::events::models::UpdateEventRequest;
use crate::services::users::db::UsersTable;
use crate::services::{SuccessResponse, TalliiResponse};

/// Creates a new Event
pub async fn create_event(
    pool: web::Data<PgPool>,
    new_event: web::Json<CreateEventRequest>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    // start the transaction
    let mut tx = pool.begin().await?;

    // create new event in the transaction
    let created_event = EventsTable::create(&mut tx, &new_event, &user).await?;

    // create the participants in the transaction
    EventMembersTable::create_many(
        &mut tx,
        &created_event.event_id,
        &user.user_id,
        &new_event.members,
    )
    .await?;

    // commit the transaction
    tx.commit().await?;

    // respond with json saying the event is created
    Ok(HttpResponse::Created().json(CreatedEventResponse {
        event_id: created_event.event_id,
    }))
}

/// Gets all Events for me
pub async fn get_me_events(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    let events = EventsTable::get_events_for_user_id(&pool, &user.user_id, "active").await?;

    Ok(HttpResponse::Ok().json(events))
}

/// Gets all Event Invitations for me
pub async fn get_me_event_invitations(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    let events = EventsTable::get_events_for_user_id(&pool, &user.user_id, "pending").await?;

    Ok(HttpResponse::Ok().json(events))
}

/// Gets all Events for a specific user
pub async fn get_users_events(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
    _user: AuthenticatedUser,
) -> TalliiResponse {
    if let Some(user) = UsersTable::get_by_username(&pool, &username).await? {
        let events = EventsTable::get_events_for_user_id(&pool, &user.user_id, "active").await?;
        Ok(HttpResponse::Ok().json(events))
    } else {
        Err(TalliiError::NOT_FOUND.default())
    }
}

/// Gets a single event for the user
pub async fn get_event(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    let event = EventsTable::get_event_by_id(&pool, &event_id).await?;

    Ok(HttpResponse::Ok().json(event))
}

/// Updates a single event
pub async fn update_event(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    event_id: web::Path<i32>,
    update_event_request: web::Json<UpdateEventRequest>,
) -> TalliiResponse {
    if let Some(_member) =
        EventMembersTable::get_member_by_user_id(&pool, &event_id, &user.user_id).await?
    {
        EventsTable::update_event_by_id(&pool, &event_id, &update_event_request).await?;
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(TalliiError::NOT_FOUND.default())
    }
}

/// Deletes a single event
pub async fn delete_event(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    if let Some(member) =
        EventMembersTable::get_member_by_user_id(&pool, &event_id, &user.user_id).await?
    {
        if member.role == String::from("admin") {
            EventsTable::delete_event_by_id(&pool, &event_id).await?;

            Ok(HttpResponse::Ok().json(SuccessResponse {
                code: String::from("EVENT_DELETED"),
                message: String::from("Event was deleted."),
            }))
        } else {
            Err(TalliiError::FORBIDDEN.default())
        }
    } else {
        Err(TalliiError::NOT_FOUND.default())
    }
}
