use actix_web::web;

use crate::services::{auth, groups};

pub fn define_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/invite-codes")
            .route(web::post().to(auth::check_invite_code))
            .route(web::get().to(auth::get_all_invite_codes)),
    )
    .service(web::resource("/invite-codes/new").route(web::post().to(auth::create_invite_codes)))
    .service(web::resource("/login").route(web::post().to(auth::login)))
    .service(web::resource("/signup").route(web::post().to(auth::signup)))
    .service(
        web::resource("/groups")
            .route(web::post().to(groups::create))
            .route(web::get().to(groups::get)),
    )
    .service(
        web::resource("/groups/{group_id}")
            .route(web::put().to(groups::update))
            .route(web::delete().to(groups::delete)),
    )
    .service(
        web::resource("/groups/{group_id}/members")
            // .route(web::get().to(groups_users::get))
            .route(web::post().to(groups::members::create))
            // .route(web::put().to(groups_users::update))
    );
}
