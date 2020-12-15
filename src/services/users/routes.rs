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

/// Resource routes getting the currently logged in users followers
pub fn users_me_followers_routes() -> Resource {
    web::resource("/me/followers").route(web::get().to(handlers::get_me_followers))
}

/// Resource routes getting the currently logged in users following
pub fn users_me_following_routes() -> Resource {
    web::resource("/me/following").route(web::get().to(handlers::get_me_following))
}

/// Resource routes following a user as the currently logged in user
pub fn users_me_follow_username_routes() -> Resource {
    web::resource("/me/follow/{username}").route(web::put().to(handlers::follow_user))
}

/// Resource routes unfollowing a user as the currently logged in user
pub fn users_me_following_username_routes() -> Resource {
    web::resource("/me/following/{username}").route(web::delete().to(handlers::unfollow_user))
}

/// Resource routes getting the events of the currently logged in user
pub fn users_me_events_routes() -> Resource {
    web::resource("/me/events").route(web::get().to(handlers::get_me_events))
}

/// Resource routes getting the event invites of the currently logged in user
pub fn users_me_events_invites_routes() -> Resource {
    web::resource("/me/events/invitations").route(web::get().to(handlers::get_me_event_invitations))
}

/// Resource routes for user username
pub fn users_username_routes() -> Resource {
    web::resource("/users/{username}")
        .route(web::get().to(handlers::get_user_by_username))
}

/// Resource routes for user username followers
pub fn users_username_followers_routes() -> Resource {
    web::resource("/users/{username}/followers")
        .route(web::get().to(handlers::get_user_followers))
}

/// Resource routes for user username following
pub fn users_username_following_routes() -> Resource {
    web::resource("/users/{username}/following")
        .route(web::get().to(handlers::get_user_followers))
}

/// Resource routes for user username events
pub fn users_username_events_routes() -> Resource {
    web::resource("/users/{username}/events")
        .route(web::get().to(handlers::get_user_events))
}

/// Resource routes for checking user email
pub fn users_email_routes() -> Resource {
    web::resource("/users/{email}")
        .route(web::get().to(handlers::check_user_email))
}

