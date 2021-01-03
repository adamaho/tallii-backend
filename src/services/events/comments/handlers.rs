use actix_web::{web, HttpResponse};

use sqlx::PgPool;

use crate::services::auth::AuthenticatedUser;
use crate::services::{SuccessResponse, TalliiResponse};

use super::db::EventCommentsTable;
use super::models::CreateEventCommentRequest;

/// Gets all comments in a single event
pub async fn get_comments(
    pool: web::Data<PgPool>,
    _user: AuthenticatedUser,
    event_id: web::Path<i32>,
) -> TalliiResponse {
    // get the members of the event
    let comments = EventCommentsTable::get_comments_by_event_id(&pool, &event_id).await?;

    // return the members of a single event
    Ok(HttpResponse::Ok().json(comments))
}

/// Creates a single comment
pub async fn create_comment(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    event_id: web::Path<i32>,
    request: web::Json<CreateEventCommentRequest>,
) -> TalliiResponse {
    // create the comment in the database
    EventCommentsTable::create_one(&pool, &user, &event_id, &request).await?;

    // response with success
    Ok(HttpResponse::Ok().json(SuccessResponse {
        code: String::from("EVENT_COMMENT_CREATED"),
        message: String::from("The provided event comment was created."),
    }))
}

/// deletes an event comment
pub async fn delete_comment(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser,
    path_params: web::Path<(i32, i32)>,
) -> TalliiResponse {
    let (_event_id, comment_id) = path_params.into_inner();

    // delete the comment
    EventCommentsTable::delete(&pool, &user, &comment_id).await?;

    // response with success
    Ok(HttpResponse::Ok().json(SuccessResponse {
        code: String::from("EVENT_COMMENT_DELETED"),
        message: String::from("The provided event comment was deleted."),
    }))
}
