use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for events
pub fn events_routes() -> Resource {
    web::resource("/events")
        .route(web::get().to(handlers::get_events))
        .route(web::post().to(handlers::create_event))
}

/// Resource routes for a specific event
pub fn events_entity_routes() -> Resource {
    web::resource("/events/{event_id}").route(web::get().to(handlers::get_event))
}
