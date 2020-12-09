use actix_web::web;

use crate::services::{friends, users};
use crate::services::events::players::routes::{players_routes, players_entity_routes};
use crate::services::events::teams::routes::{teams_routes, teams_players_routes};
use crate::services::events::routes::{events_routes, events_entity_routes};
use crate::services::users::routes::{invite_codes_routes, invite_codes_entity_routes, users_entity_routes, users_username_routes, users_email_routes};

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(invite_codes_routes())
    .service(invite_codes_entity_routes())
    .service(users_entity_routes())
    .service(users_email_routes())
    .service(users_username_routes())
    .service(web::resource("/users").route(web::get().to(users::handlers::search_users)))
    .service(web::resource("/login").route(web::post().to(users::handlers::login)))
    .service(web::resource("/signup").route(web::post().to(users::handlers::signup)))
    .service(web::resource("/friends").route(web::get().to(friends::handlers::get_friends)))
    .service(
        web::resource("/friends/invitations")
            .route(web::get().to(friends::handlers::get_friend_invitations)),
    )
    .service(
        web::resource("/friends/requests")
            .route(web::get().to(friends::handlers::get_friend_requests))
            .route(web::post().to(friends::handlers::send_friend_request)),
    )
    .service(
        web::resource("/friends/requests/cancel")
            .route(web::post().to(friends::handlers::cancel_friend_request)),
    )
    .service(
        web::resource("/friends/requests/deny")
            .route(web::post().to(friends::handlers::deny_friend_request)),
    )
    .service(
        web::resource("/friends/requests/accept")
            .route(web::post().to(friends::handlers::accept_friend_request)),
    )
    .service(events_routes())
    .service(events_entity_routes())
    .service(players_routes())
    .service(players_entity_routes())
    .service(teams_routes())
    .service(teams_players_routes());
}
