use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for event players
pub fn teams_routes() -> Resource {
    web::resource("/events/{event_id}/teams")
        .route(web::get().to(handlers::get_event_teams))
        .route(web::post().to(handlers::create_event_team))
}

/// Resource routes for a specific event player
pub fn teams_players_routes() -> Resource {
    web::resource("/events/{event_id}/teams/players")
        .route(web::get().to(handlers::get_event_team_players))
}
