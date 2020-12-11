use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::TalliiResponse;

use super::db::PlayersTable;
use super::models::PlayerRequest;

/// Gets all participants in a single event
pub async fn get_players(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    // get the players
    let players = PlayersTable::get_many(&pool, &event_id).await?;

    // return the participants
    Ok(HttpResponse::Ok().json(players))
}

/// Updates an event participant
pub async fn update_player(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    path_params: web::Path<(i32, i32)>,
    player: web::Json<PlayerRequest>,
) -> TalliiResponse {
    // get the inner tuple of params
    let (_, player_id) = path_params.into_inner();

    // update the participant
    PlayersTable::update(&pool, &player_id, &player).await?;

    // return that the player was updated
    Ok(HttpResponse::Ok().json("player updated"))
}
