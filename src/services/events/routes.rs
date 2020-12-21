use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for events
pub fn events_routes() -> Resource {
    web::resource("/events").route(web::post().to(handlers::create_event))
}

/// Resource routes for events
pub fn events_me_routes() -> Resource {
    web::resource("/me/events").route(web::get().to(handlers::get_me_events))
}

/// Resource routes for a specific event
pub fn events_entity_routes() -> Resource {
    web::resource("/events/{event_id}")
        .route(web::get().to(handlers::get_event))
        .route(web::patch().to(handlers::update_event))
        .route(web::delete().to(handlers::delete_event))
}

/// Resource routes getting the event invites of the currently logged in user
pub fn users_me_events_invites_routes() -> Resource {
    web::resource("/me/events/invitations").route(web::get().to(handlers::get_me_event_invitations))
}

/// Resource routes for user username events
pub fn users_username_events_routes() -> Resource {
    web::resource("/users/{username}/events").route(web::get().to(handlers::get_users_events))
}
