use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for wagers
pub fn me_wagers_routes() -> Resource {
    web::resource("/me/wagers").route(web::get().to(handlers::get_me_wagers))
}

/// Resource routes for wagers
pub fn me_wagers_invitations_routes() -> Resource {
    web::resource("/me/wagers/invitations").route(web::get().to(handlers::get_me_invitations))
}

/// Resource routes for a specific wagers
pub fn user_wagers_routes() -> Resource {
    web::resource("/wagers/{username}")
        .route(web::get().to(handlers::get_users_wagers))
        .route(web::post().to(handlers::create_wager))
}

/// Resource routes for a specific wager
pub fn wager_entity_routes() -> Resource {
    web::resource("/wagers/{wager_id}")
        .route(web::get().to(handlers::get_wager))
        .route(web::patch().to(handlers::update_wager_currency))
        .route(web::delete().to(handlers::decline_wager_invite))
}