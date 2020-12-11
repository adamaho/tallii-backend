use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for event players
pub fn players_routes() -> Resource {
    web::resource("/players").route(web::get().to(handlers::get_players))
}

/// Resource routes for a specific event player
pub fn players_entity_routes() -> Resource {
    web::resource("/players/{player_id}")
        .route(web::put().to(handlers::update_player))
}
