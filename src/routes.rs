use actix_web::web;

// use crate::services::events::members::routes::{players_entity_routes, players_routes};
use crate::services::events::routes::{events_entity_routes, events_me_routes, events_routes, users_me_events_invites_routes};
// use crate::services::events::teams::routes::{
//     teams_entity_routes, teams_players_routes, teams_routes,
// };
use crate::services::friends::routes::{users_me_followers_routes, users_me_following_routes, users_me_follow_username_routes, users_me_following_username_routes, users_username_followers_routes, users_username_following_routes};
use crate::services::users::routes::{invite_codes_entity_routes, invite_codes_routes, users_email_check_routes, users_login, users_me_routes, users_signup, users_username_routes, users_username_check_routes};
use crate::services::events::members::routes::event_members_routes;

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(invite_codes_routes())
        .service(invite_codes_entity_routes())
        .service(users_login())
        .service(users_signup())
        .service(users_username_check_routes())
        .service(users_email_check_routes())
        .service(users_me_routes())
        .service(users_username_routes())
        // .service(users_search_routes())
        .service(users_me_followers_routes())
        .service(users_me_follow_username_routes())
        .service(users_me_following_username_routes())
        .service(users_username_followers_routes())
        .service(users_username_following_routes())
        .service(users_me_following_routes())
        .service(events_routes())
        .service(events_me_routes())
        .service(events_entity_routes())
        .service(users_me_events_invites_routes())
        .service(event_members_routes());

        // .service(players_routes())
        // .service(players_entity_routes())
        // .service(teams_routes())
        // .service(teams_players_routes())
        // .service(teams_entity_routes());
}
