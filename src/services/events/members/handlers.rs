use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::TalliiResponse;

use super::db::EventMembersTable;
use crate::services::events::members::models::UpdateMemberRequest;
use crate::services::users::db::UsersTable;
use actix_web::error::PayloadError::Http2Payload;
use crate::errors::TalliiError;

/// Gets all members in a single event
pub async fn get_members(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>
) -> TalliiResponse {
    // get the members of the event
    let members = EventMembersTable::get_members_by_event_id(&pool, &event_id).await?;

    // return the members of a single event
    Ok(HttpResponse::Ok().json(members))
}

/// Updates an event member
pub async fn update_member(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path_params: web::Path<(i32, i32)>,
    update_member_request: web::Json<UpdateMemberRequest>,
) -> TalliiResponse {
    let (event_id, user_id) = path_params.into_inner();

    // check to make sure user is member of event
    if let Some(member) = EventMembersTable::get_member_by_user_id(&pool, &event_id, &user.user_id).await {

        // check to make sure user is admin or themselves
        if member.role == String::from("admin") || user.user_id == user_id {
            EventMembersTable::update(&pool, &user_id, &event_id, &update_member_request).await;

            Ok(HttpResponse::NoContent().finish())
        } else {
            Err(TalliiError::FORBIDDEN.default())
        }
    } else {
        Err(TalliiError::FORBIDDEN.default())
    }
}

/// deletes an event member
pub async fn delete_member(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path_params: web::Path<(i32, i32)>,
) -> TalliiResponse {
    let (event_id, user_id) = path_params.into_inner();

    // check to make sure user is member of event
    if let Some(member) = EventMembersTable::get_member_by_user_id(&pool, &event_id, &user.user_id).await {

        // check to make sure user is admin or themselves
        if member.role == String::from("admin") || user.user_id == user_id {
            EventMembersTable::delete(&pool, &user_id, &event_id).await;

            Ok(HttpResponse::NoContent().finish())
        } else {
            Err(TalliiError::FORBIDDEN.default())
        }
    } else {
        Err(TalliiError::FORBIDDEN.default())
    }
}
