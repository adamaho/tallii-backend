use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::friends::db::FriendRepository;
use crate::services::friends::models::{FriendRequest, FriendRequestAcceptance};
use crate::services::TalliiResponse;

/// Creates a new friend invite for the requesting user
pub async fn send_friend_request(
    pool: web::Data<PgPool>,
    new_friend: web::Json<FriendRequest>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    FriendRepository::create_friend_request(&pool, &new_friend, &user).await?;

    Ok(HttpResponse::Created().json(""))
}

/// Accepts a new friend invite for the requesting user
pub async fn accept_friend_request(
    pool: web::Data<PgPool>,
    new_friend: web::Json<FriendRequestAcceptance>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    FriendRepository::accept_friend_request(&pool, &new_friend, &user).await?;

    Ok(HttpResponse::Created().json(""))
}

// Gets a list of all current friends for the requesting user
pub async fn get_friends(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    let friends = FriendRepository::get_many(&pool, &user).await?;

    Ok(HttpResponse::Ok().json(friends))
}

// Gets a list of all current friends for the requesting user
pub async fn get_friend_requests(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    let requests = FriendRepository::get_many_requests(&pool, &user).await?;

    Ok(HttpResponse::Ok().json(requests))
}

// Gets a list of all current friends for the requesting user
pub async fn get_friend_invitations(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    let invitations = FriendRepository::get_many_invitations(&pool, &user).await?;

    Ok(HttpResponse::Ok().json(invitations))
}
