use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::TalliiResponse;

use super::db::PlayersTable;
use super::models::PlayerQueryParams;
use crate::services::events::players::models::UpdatePlayerRequest;

/// Gets all participants in a single event
pub async fn get_players(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    params: web::Query<PlayerQueryParams>,
) -> TalliiResponse {
    // get the players
    let players = PlayersTable::get_many(&pool, &params.event_id).await?;

    // return the participants
    Ok(HttpResponse::Ok().json(players))
}

/// Updates an event participant
pub async fn update_player(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    player_id: web::Path<i32>,
    player: web::Json<UpdatePlayerRequest>,
) -> TalliiResponse {
    // update the participant
    PlayersTable::update(&pool, &player_id, &player).await?;

    // return that the player was updated
    Ok(HttpResponse::Ok().finish())
}
