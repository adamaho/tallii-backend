use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for event players
pub fn event_teams_routes() -> Resource {
    web::resource("/events/{event_id}/teams")
        .route(web::get().to(handlers::get_teams))
        .route(web::post().to(handlers::create_team))
}

/// Resource routes for teams entity
pub fn event_teams_entity_routes() -> Resource {
    web::resource("/events/{event_id}/teams/{team_id}")
        .route(web::patch().to(handlers::update_team))
        .route(web::delete().to(handlers::delete_team))
}

/// Resource routes for a specific event player
pub fn event_teams_members_routes() -> Resource {
    web::resource("/events/{event_id}/teams/{team_id}/members")
        .route(web::get().to(handlers::get_team_members))
}

/// Resource routes for a specific event player
pub fn event_teams_members_entity_routes() -> Resource {
    web::resource("/events/{event_id}/teams/{team_id}/members/{user_id}")
        .route(web::delete().to(handlers::delete_team_member))
        .route(web::put().to(handlers::add_team_member))
}
