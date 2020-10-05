use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::services::friends::db::FriendRepository;
use crate::services::friends::models::{FriendRequest, FriendRequestAcceptance};
use crate::services::{AuthenticatedUser, TalliiResponse};

/// Creates a new friend invite for the requesting user
pub async fn send_friend_request(
    pool: web::Data<PgPool>,
    new_friend: web::Json<FriendRequest>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    FriendRepository::create_friend_request(&pool, &new_friend, &user).await?;

    Ok(HttpResponse::Created().finish())
}

/// Accepts a new friend invite for the requesting user
pub async fn accept_friend_request(
    pool: web::Data<PgPool>,
    new_friend: web::Json<FriendRequestAcceptance>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    FriendRepository::accept_friend_request(&pool, &new_friend, &user).await?;

    Ok(HttpResponse::Created().finish())
}
