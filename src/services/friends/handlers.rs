use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::friends::db::FriendsTable;

use crate::errors::TalliiError;
use crate::services::users::db::UsersTable;
use crate::services::{SuccessResponse, TalliiResponse};

/// Gets the followers of me
pub async fn get_me_followers(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    // get all users that me follows
    let followers = FriendsTable::get_followers_by_id(&pool, &user.user_id).await?;

    Ok(HttpResponse::Ok().json(followers))
}

/// Gets the users me is following
pub async fn get_me_following(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    // get all users that me follows
    let following = FriendsTable::get_following_by_id(&pool, &user.user_id).await?;

    Ok(HttpResponse::Ok().json(following))
}

/// Follows the provided user matching the username
pub async fn follow_user(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    if let Some(friend) = UsersTable::get_by_username(&pool, &username).await? {
        FriendsTable::follow_user_by_id(&pool, &user.user_id, &friend.user_id).await?;

        Ok(HttpResponse::Ok().json(SuccessResponse {
            code: String::from("FOLLOWED_USER"),
            message: String::from("The provided user has been followed."),
        }))
    } else {
        Err(TalliiError::NOT_FOUND.default())
    }
}

/// Unfollows the provided user matching the username
pub async fn unfollow_user(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    if let Some(friend) = UsersTable::get_by_username(&pool, &username).await? {
        FriendsTable::unfollow_user_by_id(&pool, &user.user_id, &friend.user_id).await?;

        Ok(HttpResponse::Ok().json(SuccessResponse {
            code: String::from("UNFOLLOWED_USER"),
            message: String::from("The provided user has been unfollowed."),
        }))
    } else {
        Err(TalliiError::NOT_FOUND.default())
    }
}

/// Gets the followers of the user matching the provided username
pub async fn get_user_followers(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
    _user: AuthenticatedUser,
) -> TalliiResponse {
    if let Some(user) = UsersTable::get_by_username(&pool, &username).await? {
        let followers = FriendsTable::get_followers_by_id(&pool, &user.user_id).await?;

        Ok(HttpResponse::Ok().json(followers))
    } else {
        Err(TalliiError::NOT_FOUND.default())
    }
}

/// Gets the users the provided username is following
pub async fn get_user_following(
    pool: web::Data<PgPool>,
    username: web::Path<String>,
    _user: AuthenticatedUser,
) -> TalliiResponse {
    if let Some(user) = UsersTable::get_by_username(&pool, &username).await? {
        let following = FriendsTable::get_following_by_id(&pool, &user.user_id).await?;

        Ok(HttpResponse::Ok().json(following))
    } else {
        Err(TalliiError::NOT_FOUND.default())
    }
}
