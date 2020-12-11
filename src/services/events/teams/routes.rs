use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for event players
pub fn teams_routes() -> Resource {
    web::resource("/teams")
        .route(web::get().to(handlers::get_teams))
        .route(web::post().to(handlers::create_team))
}

// /// Resource routes for teams entity
// pub fn teams_entity_routes() -> Resource {
//     web::resource("/teams/{team_id}")
//         .route(web::get().to(handlers::get_team))
// }

/// Resource routes for a specific event player
pub fn teams_players_routes() -> Resource {
    web::resource("/teams/{team_id}/players")
        .route(web::get().to(handlers::get_team_players))
}
