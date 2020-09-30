use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use super::{AuthenticatedUser, TalliiResponse};

use crate::errors::TalliiError;
use crate::models::group_member::NewGroupMember;
use crate::repositories::group_member::GroupMembersRepository;

/// Creates a new group member
pub async fn create(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    group_id: web::Path<i32>,
    member: web::Json<NewGroupMember>,
) -> TalliiResponse {
    // assign the inner i32 to a new spot in memory
    let id = group_id.into_inner();

    // check to make sure the user is an owner of the group before updating it
    if GroupMembersRepository::check_ownership(&pool, &user, id).await? == false {
        return Err(TalliiError::UNAUTHORIZED.default());
    }

    GroupMembersRepository::create_one(&pool, id, &member).await?;

    Ok(HttpResponse::Created().finish())
}

/// Gets all the members in a group
pub async fn get(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    group_id: web::Path<i32>,
) -> TalliiResponse {
    // assign the inner i32 to a new spot in memory
    let id = group_id.into_inner();

    // check if user is a part of the group before fetching all of the members
    if GroupMembersRepository::check_membership(&pool, &user, id).await? == false {
        return Err(TalliiError::UNAUTHORIZED.default());
    }
    // get all of the group users
    let all_members = GroupMembersRepository::get_many(&pool, id).await?;

    Ok(HttpResponse::Ok().json(all_members))
}
