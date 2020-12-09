use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;

use crate::services::events::db::EventsTable;

use crate::services::events::models::{
    Event,
    CreatedEventResponse,
    EventQueryParams,
    NewEvent
};

use crate::services::events::players::db::EventsPlayersTable;

use crate::services::TalliiResponse;

/// Creates a new Event
pub async fn create_event(
    pool: web::Data<PgPool>,
    new_event: web::Json<NewEvent>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    // start the transaction
    let mut tx = pool.begin().await?;

    // create new event in the transaction
    let created_event = EventsTable::create(&mut tx, &new_event, &user).await?;

    // create the participants in the transaction
    EventsPlayersTable::create_many(
        &mut tx,
        &created_event.event_id,
        &user.user_id,
        &new_event.participants,
    )
    .await?;

    // commit the transaction
    tx.commit().await?;

    // respond with json saying the event is created
    Ok(HttpResponse::Created().json(CreatedEventResponse { event_id: created_event.event_id }))
}

/// Gets all Events for the user
pub async fn get_events(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    params: web::Query<EventQueryParams>,
) -> TalliiResponse {
    let events = EventsTable::get_many(&pool, &user, &params).await?;

    Ok(HttpResponse::Ok().json(events))
}

/// Gets a single event for the user
pub async fn get_event(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    let event = EventsTable::get_one(&pool, &event_id).await?;

    Ok(HttpResponse::Ok().json(event))
}
