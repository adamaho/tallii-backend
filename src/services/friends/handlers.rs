use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::friends::db::FriendsTable;
use crate::services::friends::models::{FriendOperation, FriendQueryParams, FriendRequest};

use crate::services::TalliiResponse;

/// Gets a list of friends that match the provided query param
pub async fn get_friends(
    pool: web::Data<PgPool>,
    params: web::Query<FriendQueryParams>,
    _user: AuthenticatedUser,
) -> TalliiResponse {
    let friends = FriendsTable::get_many(&pool, &params).await?;

    Ok(HttpResponse::Ok().json(friends))
}

/// creates, accepts, denies, or cancels friend requests
pub async fn post_friends(
    pool: web::Data<PgPool>,
    friend: web::Json<FriendRequest>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    // handle the request based on the operation
    match &friend.operation {
        FriendOperation::SendRequest => {
            FriendsTable::create_friend_request(&pool, &friend, &user).await?;
        }
        FriendOperation::AcceptRequest => {
            FriendsTable::accept_friend_request(&pool, &friend, &user).await?;
        }
        FriendOperation::DenyRequest => {
            FriendsTable::deny_friend_request(&pool, &friend, &user).await?;
        }
        FriendOperation::CancelRequest => {
            FriendsTable::cancel_friend_request(&pool, &friend, &user).await?;
        }
    }

    Ok(HttpResponse::Ok().finish())
}
