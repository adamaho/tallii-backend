use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use super::{AuthenticatedUser, TalliiResponse};

use crate::models::group::{NewGroup, GroupResponsePayload};
use crate::repositories::group::GroupRepository;
use crate::repositories::group_users::GroupUsersRepository;

/// Creates a new group
pub async fn create_group(
    pool: web::Data<PgPool>,
    new_group: web::Json<NewGroup>,
    _user: AuthenticatedUser,
) -> TalliiResponse {
    // start the transaction
    let mut tx = pool.begin().await?;

    // create new group in the transaction
    let created_group = GroupRepository::create(&mut tx, &new_group).await?;

    // create a new group with the owner being the current user
    let created_group_users = GroupUsersRepository::create(
        &mut tx,
        created_group.group_id,
        &new_group.members,
    )
    .await?;

    tx.commit().await?;

    // combine users and group together to form final response
    let response = GroupResponsePayload {
        group_id: created_group.group_id,
        name: created_group.name,
        description: created_group.description,
        avatar: created_group.avatar,
        members: created_group_users,
        created_at: created_group.created_at
    };

    Ok(HttpResponse::Ok().json(response))
}

/// Gets all groups that are associated with the requesting user
pub async fn get_groups(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    let groups = GroupRepository::get_by_user_id(&pool, &user).await?;
    Ok(HttpResponse::Ok().json(groups))
}

/// Creates a new group
pub async fn update_group(pool: web::Data<PgPool>) -> TalliiResponse {
    Ok(HttpResponse::Ok().finish())
}

/// Creates a new group
pub async fn delete_group(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    Ok(HttpResponse::Ok().finish())
}
