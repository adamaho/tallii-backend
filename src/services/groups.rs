use std::ops::Deref;

use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use super::{AuthenticatedUser, TalliiResponse};

use crate::models::group::NewGroup;
use crate::repositories::group::GroupRepository;
use crate::repositories::group_users::GroupUsersRepository;

/// Creates a new group
pub async fn create_group(
    pool: web::Data<PgPool>,
    new_group: web::Data<NewGroup>,
    user: AuthenticatedUser,
) -> TalliiResponse {

    // start the transaction
    let tx = pool.begin().await?;

    // group repo
    let group_repo = GroupRepository::new(&mut tx);

    // group users
    let group_users_repo = GroupUsersRepository::new(tx.clone());

    // create new group
    let new_created_group = group_repo.create_group(&new_group).await?;

    // create group user with owner being the current user
    let new_group_users = group_users_repo
        .create_group_users(&user, &new_created_group.group_id, &new_group.members)
        .await?;

    // combine users and group together to form final response

    Ok(HttpResponse::Ok().json(new_created_group))
}

/// Creates a new group
pub async fn get_groups(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    println!("made it past");
    Ok(HttpResponse::Ok().finish())
}

/// Creates a new group
pub async fn update_group(pool: web::Data<PgPool>) -> TalliiResponse {
    Ok(HttpResponse::Ok().finish())
}

/// Creates a new group
pub async fn delete_group(pool: web::Data<PgPool>, user: AuthenticatedUser) -> TalliiResponse {
    Ok(HttpResponse::Ok().finish())
}
