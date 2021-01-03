use sqlx::PgPool;

use crate::errors::TalliiError;

use super::models::{CreateEventCommentRequest, EventComment};
use crate::services::auth::AuthenticatedUser;

pub struct EventCommentsTable;

impl EventCommentsTable {
    /// Gets all comments on a specific event
    pub async fn get_comments_by_event_id(
        pool: &PgPool,
        event_id: &i32,
    ) -> Result<Vec<EventComment>, TalliiError> {
        let comments = sqlx::query_as::<_, EventComment>(
            r#"
                select
                    *
                from
                    events_comments
                where
                    event_id = $1
            "#,
        )
        .bind(event_id)
        .fetch_all(pool)
        .await?;

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
