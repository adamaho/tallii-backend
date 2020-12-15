use actix_web::{web, Resource};

use super::handlers;

/// Resource routes for friends
pub fn friends_routes() -> Resource {
    web::resource("/friends").route(web::get().to(handlers::get_friends))
}

/// Resource routes for me friends
pub fn friends_me_routes() -> Resource {
    web::resource("/me/friends")
        .route(web::get().to(handlers::get_me_friends))
        .route(web::post().to(handlers::post_me_friends))
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
