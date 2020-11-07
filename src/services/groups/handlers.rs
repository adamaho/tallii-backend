use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::errors::TalliiError;
use crate::services::auth::AuthenticatedUser;
use crate::services::groups::db::{GroupMembersRepository, GroupRepository};
use crate::services::groups::models::{EditGroup, NewGroup, NewGroupMember};
use crate::services::TalliiResponse;

/// Creates a new group
pub async fn create(
    pool: web::Data<PgPool>,
    new_group: web::Json<NewGroup>,
    user: AuthenticatedUser,
) -> TalliiResponse {
    // start the transaction
    let mut tx = pool.begin().await?;

    // create new group in the transaction
    let created_group = GroupRepository::create(&mut tx, &new_group).await?;

    let admin_member = NewGroupMember {
        user_id: user.user_id,
        role: String::from("admin")
    };

    // create a new group with the owner being the current user
    GroupMembersRepository::create_one_tx(&mut tx, created_group.group_id, &admin_member)
        .await?;

    tx.commit().await?;

    Ok(HttpResponse::Created().json(created_group))
}

/// Gets all groups that are associated with the requesting user
pub async fn get(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    let groups = GroupRepository::get_by_user_id(&pool, &user).await?;
    Ok(HttpResponse::Ok().json(groups))
}

/// Updates a new group
pub async fn update(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    group_id: web::Path<i32>,
    group: web::Json<EditGroup>,
) -> TalliiResponse {
    // assign the inner i32 to a new spot in memory
    let id = group_id.into_inner();

    // check to make sure the user is an owner of the group before updating it
    if GroupMembersRepository::check_ownership(&pool, &user, id).await? == false {
        return Err(TalliiError::UNAUTHORIZED.default());
    }

    // update the group
    GroupRepository::update(&pool, id, &group).await?;

    Ok(HttpResponse::Ok().json(""))
}

/// Deletes a group
pub async fn delete(
    user: AuthenticatedUser,
    pool: web::Data<PgPool>,
    group_id: web::Path<i32>,
) -> TalliiResponse {
    // assign the inner i32 to a new spot in memory
    let id = group_id.into_inner();

    // check to make sure the user is an owner of the group before updating it
    if GroupMembersRepository::check_ownership(&pool, &user, id).await? == false {
        return Err(TalliiError::UNAUTHORIZED.default());
    }

    // delete the group
    GroupRepository::delete(&pool, id).await?;

    Ok(HttpResponse::Ok().json(""))
}

/// Creates a new group member
pub async fn create_member(
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

    Ok(HttpResponse::Created().json(""))
}

/// Gets all the members in a group
pub async fn get_members(
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
