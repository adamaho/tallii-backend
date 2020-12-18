use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgConnection, PgQueryAs};
use sqlx::{PgPool, Transaction};

use crate::errors::TalliiError;

use super::models::EventMember;
use crate::services::events::members::models::{
    EventMemberResponse, InviteMemberRequest, MemberExists, UpdateMemberRequest,
};

pub struct EventMembersTable;

impl EventMembersTable {
    /// Creates many event members in the database
    pub async fn create_many(
        tx: &mut Transaction<PoolConnection<PgConnection>>,
        event_id: &i32,
        user_id: &i32,
        members: &Vec<i32>,
    ) -> Result<(), TalliiError> {
        // init the query
        let mut query =
            String::from("insert into events_members (event_id, user_id, state, role) values");

        // add the current user to the players
        query.push_str(&format!("({}, {}, 'active', 'admin')", event_id, user_id));

        // create the queries for each of the new players and add them to the query string
        for (i, user_id) in members.iter().enumerate() {
            // if we are appending values onto the query we need to separate them with commas
            if i <= members.len() - 1 {
                query.push_str(",")
            }

            query.push_str(&format!("({}, {}, 'pending', 'member')", event_id, user_id));
        }

        // execute the query
        sqlx::query(&query).execute(tx).await?;

        Ok(())
    }

    /// Creates a single event member in the database
    pub async fn create_one(
        pool: &PgPool,
        event_id: &i32,
        member: &InviteMemberRequest,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                insert into events_members (event_id, user_id, state, role) values ($1, $2, 'pending', 'member')
            "#
        )
            .bind(event_id)
            .bind(member.user_id)
            .execute(pool)
            .await?;

        Ok(())
    }

    /// Checks if a member exists in the database
    pub async fn exists(pool: &PgPool, event_id: &i32, user_id: &i32) -> Result<bool, TalliiError> {
        let exists = sqlx::query_as::<_, MemberExists>(
            r#"
                select
                    exists (
                        select
                            1
                        from
                            events_members
                        where
                            event_id = $1
                        and
                            user_id = $2
                    )
            "#,
        )
        .bind(event_id)
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(exists.exists)
    }

    /// Gets a member by an event_id and user_id
    pub async fn get_member_by_user_id(
        pool: &PgPool,
        event_id: &i32,
        user_id: &i32,
    ) -> Result<Option<EventMember>, TalliiError> {
        let member = sqlx::query_as::<_, EventMember>(
            r#"
                select
                    member_id,
                    event_id,
                    user_id,
                    state,
                    role,
                    created_at
                from
                    events_members
                where
                    event_id = $1
                and
                    user_id = $2
            "#,
        )
        .bind(event_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(member)
    }

    /// Gets all members for a single event
    pub async fn get_members_by_event_id(
        pool: &PgPool,
        event_id: &i32,
    ) -> Result<Vec<EventMemberResponse>, TalliiError> {
        let members = sqlx::query_as::<_, EventMemberResponse>(
            r#"
                select
                    u.user_id,
                    u.username,
                    u.avatar,
                    u.taunt
                from
                    events_members em
                inner join
                    users u
                on
                    em.user_id = u.user_id
                where
                    event_id = $1
            "#,
        )
        .bind(event_id)
        .fetch_all(pool)
        .await?;

        Ok(members)
    }

    /// Updates a single member
    pub async fn update(
        pool: &PgPool,
        user_id: &i32,
        event_id: &i32,
        member: &UpdateMemberRequest,
    ) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                update
                    events_members
                set
                    state = $1,
                    role = $2
                where
                    user_id = $3
                and
                    event_id = $4
            "#,
        )
        .bind(&member.state)
        .bind(&member.role)
        .bind(user_id)
        .bind(event_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Deletes a single member
    pub async fn delete(pool: &PgPool, user_id: &i32, event_id: &i32) -> Result<(), TalliiError> {
        sqlx::query(
            r#"
                delete from
                    events_members
                where
                    user_id = $1
                and
                    event_id = $2
            "#,
        )
        .bind(user_id)
        .bind(event_id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
