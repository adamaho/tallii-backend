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
            .route(web::post().to(groups::create_group))
            .route(web::get().to(groups::get_groups)),
    )
    .service(
        web::resource("/groups/{group_id}")
            .route(web::put().to(groups::update_group))
            .route(web::delete().to(groups::delete_group)),
    )
    .service(
        web::resource("/groups/{group_id}/members")
            // .route(web::get().to(groups_users::get_groups_users))
            // .route(web::post().to(groups_users::add_groups_user))
            // .route(web::put().to(groups_users::update_groups_user))
    );
}
