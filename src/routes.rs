use actix_web::web;

use crate::services::events::players::routes::{players_entity_routes, players_routes};
use crate::services::events::routes::{events_entity_routes, events_routes};
use crate::services::events::teams::routes::{teams_players_routes, teams_routes};
use crate::services::friends::routes::friends_routes;
use crate::services::users::routes::{
    invite_codes_entity_routes, invite_codes_routes, users_email_routes, users_entity_routes,
    users_login, users_signup, users_username_routes,
};
use crate::services::{friends, users};

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(invite_codes_routes())
        .service(invite_codes_entity_routes())
        .service(users_login())
        .service(users_signup())
        .service(users_entity_routes())
        .service(users_email_routes())
        .service(users_username_routes())
        .service(friends_routes())
        .service(web::resource("/users").route(web::get().to(users::handlers::search_users)))
        .service(events_routes())
        .service(events_entity_routes())
        .service(players_routes())
        .service(players_entity_routes())
        .service(teams_routes())
        .service(teams_players_routes());
}
