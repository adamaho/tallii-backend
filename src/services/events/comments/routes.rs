use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for event members
pub fn event_comments() -> Resource {
    web::resource("/events/{event_id}/comments")
        .route(web::post().to(handlers::create_comment))
        .route(web::get().to(handlers::get_comments))
}

/// Resource routes for event members
pub fn event_comments_entity() -> Resource {
    web::resource("/events/{event_id}/comments/{comment_id}")
        .route(web::delete().to(handlers::delete_comment))
}
