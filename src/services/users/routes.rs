use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for invite codes
pub fn invite_codes_routes() -> Resource {
    web::resource("/invite-codes")
        .route(web::get().to(handlers::get_invite_codes))
        .route(web::post().to(handlers::create_invite_codes))
}

/// Resource routes for specific invite code
pub fn invite_codes_entity_routes() -> Resource {
    web::resource("/invite-codes/{invite_code}").route(web::get().to(handlers::check_invite_code))
}

/// Resource routes for logging in
pub fn users_login() -> Resource {
    web::resource("/login").route(web::post().to(handlers::login))
}

/// Resource routes for signing up
pub fn users_signup() -> Resource {
    web::resource("/signup").route(web::post().to(handlers::signup))
}

/// Resource routes for getting the currently logged in user
pub fn users_me_routes() -> Resource {
    web::resource("/me").route(web::get().to(handlers::get_me))
}

/// Resource routes for user username
pub fn users_username_check_routes() -> Resource {
    web::resource("/users/check-username/{username}").route(web::get().to(handlers::check_username))
}

/// Resource routes for checking user email
pub fn users_email_check_routes() -> Resource {
    web::resource("/users/check-email/{email}").route(web::get().to(handlers::check_user_email))
}

/// Resource routes for user username
pub fn users_username_routes() -> Resource {
    web::resource("/users/{username}").route(web::get().to(handlers::get_user_by_username))
}

/// Resource routes for user username search
pub fn users_username_search_routes() -> Resource {
    web::resource("/search/users").route(web::get().to(handlers::search_users))
}
