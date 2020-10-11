use actix_web::web;

use crate::services::{friends, groups, users};

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/invite-codes")
            .route(web::post().to(users::handlers::check_invite_code))
            .route(web::get().to(users::handlers::get_all_invite_codes)),
    )
    .service(
        web::resource("/invite-codes/new")
            .route(web::post().to(users::handlers::create_invite_codes)),
    )
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
        web::resource("/friends/requests/accept")
            .route(web::post().to(friends::handlers::accept_friend_request)),
    )
    .service(
        web::resource("/groups")
            .route(web::post().to(groups::handlers::create))
            .route(web::get().to(groups::handlers::get)),
    )
    .service(
        web::resource("/groups/{group_id}")
            .route(web::put().to(groups::handlers::update))
            .route(web::delete().to(groups::handlers::delete)),
    )
    .service(
        web::resource("/groups/{group_id}/members")
            .route(web::get().to(groups::handlers::get_members))
            .route(web::post().to(groups::handlers::create_member)), // .route(web::put().to(groups_users::update)),
    );
}
