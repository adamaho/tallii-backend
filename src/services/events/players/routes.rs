use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for event players
pub fn players_routes() -> Resource {
    web::resource("/events/{event_id}/players").route(web::get().to(handlers::get_event_players))
}

/// Resource routes for a specific event player
pub fn players_entity_routes() -> Resource {
    web::resource("/events/{event_id}/players/{event_player_id}")
        .route(web::put().to(handlers::update_event_player))
}
