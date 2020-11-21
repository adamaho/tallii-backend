use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::generics::PaginatedResponse;
use crate::services::auth::AuthenticatedUser;
use crate::services::friends::db::FriendRepository;
use crate::services::friends::models::{
    FriendQueryParams, FriendRequest, FriendRequestAcceptance, FriendRequestDeny,
};
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

/// Denies a new friend request that was received
pub async fn deny_friend_request(
    pool: web::Data<PgPool>,
    requested_friend: web::Json<FriendRequestDeny>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    FriendRepository::deny_friend_request(&pool, &requested_friend, &user).await?;

    Ok(HttpResponse::Ok().json(""))
}

/// Cancels a friend request that was sent
pub async fn cancel_friend_request(
    pool: web::Data<PgPool>,
    sent_friend: web::Json<FriendRequestDeny>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    FriendRepository::cancel_friend_request(&pool, &sent_friend, &user).await?;

    Ok(HttpResponse::Ok().json(""))
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
pub async fn get_friends(
    pool: web::Data<PgPool>,
    params: web::Query<FriendQueryParams>,
    _user: AuthenticatedUser,
) -> TalliiResponse {
    let friends = FriendRepository::get_many(&pool, &params).await?;

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
