use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::TalliiResponse;

use super::db::EventsPlayersTable;
use super::models::EventPlayerRequest;

/// Gets all participants in a single event
pub async fn get_event_players(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    // get the players
    let players = EventsPlayersTable::get_many(&pool, &event_id).await?;

    // return the participants
    Ok(HttpResponse::Ok().json(players))
}

/// Updates an event participant
pub async fn update_event_player(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    path_params: web::Path<(i32, i32)>,
    player: web::Json<EventPlayerRequest>,
) -> TalliiResponse {
    // get the inner tuple of params
    let (_, event_player_id) = path_params.into_inner();

    // update the participant
    EventsPlayersTable::update(&pool, &event_player_id, &player).await?;

    // return that the player was updated
    Ok(HttpResponse::Ok().json("player updated"))
}
