use sqlx::PgPool;

use crate::errors::TalliiError;

use super::models::{CreateEventCommentRequest, EventCommentResponse, EventCommentRow};
use crate::services::auth::AuthenticatedUser;
use crate::services::users::models::PublicUser;

pub struct EventCommentsTable;

impl EventCommentsTable {
    /// Gets all comments on a specific event
    pub async fn get_comments_by_event_id(
        pool: &PgPool,
        event_id: &i32,
    ) -> Result<Vec<EventCommentResponse>, TalliiError> {
        let comments = sqlx::query_as::<_, EventCommentRow>(
            r#"
                select
                    ec.comment_id,
                    ec.event_id,
                    ec.user_id,
                    ec.comment,
                    ec.created_at,
                    u.emoji,
                    u.bg_color,
                    u.username,
                    u.bio
                from
                    events_comments ec
                left join
                    users u
                on
                    ec.user_id = u.user_id
                where
                    ec.event_id = $1
            "#,
        )
        .bind(event_id)
        .fetch_all(pool)
        .await?
            .into_iter()
            .map(| row | {
                return EventCommentResponse {
                    comment_id: row.comment_id,
                    event_id: row.event_id,
                    user: PublicUser {
                        user_id: row.user_id,
                        username: row.username,
                        emoji: row.emoji,
                        bg_color: row.bg_color,
                        bio: row.bio
                    },
                    comment: row.comment,
                    created_at: row.created_at,
                }
            })
            .collect();

        Ok(comments)
    }

    /// Creates a single event comment in the database
    pub async fn create_one(
        pool: &PgPool,
        user: &AuthenticatedUser,
        event_id: &i32,
        request: &CreateEventCommentRequest,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                insert into
                    events_comments (event_id, user_id, comment)
                values
                    ($1, $2, $3)
            "#,
        )
        .bind(event_id)
        .bind(user.user_id)
        .bind(&request.comment)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Deletes a single comment
    pub async fn delete(
        pool: &PgPool,
        user: &AuthenticatedUser,
        comment_id: &i32,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                delete from
                    events_comments
                where
                    comment_id = $1
                and
                    user_id = $2
            "#,
        )
        .bind(comment_id)
        .bind(user.user_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
