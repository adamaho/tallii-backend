use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use super::{AuthenticatedUser, TalliiResponse};

use crate::errors::TalliiError;
use crate::models::group::{EditGroup, GroupResponsePayload, NewGroup};
use crate::repositories::group::GroupRepository;
use crate::repositories::group_member::GroupMembersRepository;

pub mod members;

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

    // create a new group with the owner being the current user
    let created_group_users =
        GroupMembersRepository::create_many(&mut tx, &user, created_group.group_id, &new_group.members)
            .await?;

    tx.commit().await?;

    // combine users and group together to form final response
    let response = GroupResponsePayload {
        group_id: created_group.group_id,
        name: created_group.name,
        description: created_group.description,
        avatar: created_group.avatar,
        members: created_group_users,
        created_at: created_group.created_at,
    };

    Ok(HttpResponse::Created().json(response))
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
    let updated_group = GroupRepository::update(&pool, id, &group).await?;

    Ok(HttpResponse::Ok().json(updated_group))
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

    Ok(HttpResponse::Ok().finish())
}