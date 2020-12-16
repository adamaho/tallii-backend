use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for event members
pub fn members_routes() -> Resource {
    web::resource("/events/{event_id}/members").route(web::get().to(handlers::get_members))
}

/// Resource routes for event members
pub fn members_user_routes() -> Resource {
    web::resource("/events/{event_id}/members/{user_id}")
        .route(web::put().to(handlers::update_member))
        .route(web::delete().to(handlers::delete_member))
}

// /// Resource routes for a specific event player
// pub fn players_entity_routes() -> Resource {
//     web::resource("/players/{player_id}").route(web::put().to(handlers::update_player))
// }
