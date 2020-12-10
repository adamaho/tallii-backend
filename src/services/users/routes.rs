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

/// Resource routes for checking user email
pub fn users_entity_routes() -> Resource {
    web::resource("/users/{user_id}").route(web::get().to(handlers::get_user))
}

/// Resource routes for checking user email
pub fn users_email_routes() -> Resource {
    web::resource("/users/check-email/{email}").route(web::get().to(handlers::check_email))
}

/// Resource routes for checking username
pub fn users_username_routes() -> Resource {
    web::resource("/users/check-username/{username}").route(web::get().to(handlers::check_username))
}

/// Resource routes for logging in
pub fn users_login() -> Resource {
    web::resource("/login").route(web::post().to(handlers::login))
}

/// Resource routes for signing up
pub fn users_signup() -> Resource {
    web::resource("/signup").route(web::post().to(handlers::signup))
}
