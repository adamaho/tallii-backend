use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for friends
pub fn friends_routes() -> Resource {
    web::resource("/friends")
        .route(web::get().to(handlers::get_friends))
        .route(web::post().to(handlers::post_friends))
}
