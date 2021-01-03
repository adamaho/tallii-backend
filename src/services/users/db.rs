use futures::future::try_join_all;
use nanoid::generate;
use rand::prelude::*;
use sqlx::PgPool;

use crate::crypto::Crypto;
use crate::errors::TalliiError;

use super::models::{InviteCode, NewUser, PublicUser, User, UserQuery};

/// Emoji list for users
#[allow(clippy::non_ascii_literal)]
const EMOJIS: &[&str] = &[
    "😀", "😁", "😂", "🤣", "😃", "😄", "😅", "😆", "😉", "😊", "😋", "😎", "🙂", "🤗", "🤩", "🤔",
    "🤨", "😐", "😑", "😶", "🙄", "😏", "😴", "😌", "😒", "🙃", "😲", "🤯", "😬", "🥵", "🥶", "😳",
    "🤪", "🤠", "🤡", "🥳", "🥴", "🥺", "🧐", "🤓", "😈", "👿", "👹", "👺", "💀", "👻", "👽", "🤖",
    "😺", "😸", " ", "😼", "😽", "🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🦝", "🐻", "🐼", "🦘", "🦡",
    "🐨", "🐯", "🦁", "🐮", "🐷", "🐽", "🐸", "🐵", "🙈", "🙉", "🙊", "🐒", "🐔", "🐧", "🐦", "🐤",
];

/// Color list for users
const BACKGROUNDS: &[&str] = &[
    "#F3F4F6", "#FEE2E2", "#FEF3C7", "#D1FAE5", "#DBEAFE", "#E0E7FF", "#EDE9FE", "#FCE7F3",
];

/// Represents the avatar of the user
pub struct Avatar {
    background: &'static str,
    emoji: &'static str,
}

impl Avatar {
    pub fn new() -> Avatar {
        // init the rng thread
        let mut rng = rand::thread_rng();

        // get a random emoji
        let emoji = EMOJIS[rng.gen_range(0..EMOJIS.len())];

        // get random bg
        let background = BACKGROUNDS[rng.gen_range(0..BACKGROUNDS.len())];

        return Avatar { emoji, background };
    }
}

pub struct InviteCodesTable;

impl InviteCodesTable {
    /// Checks if the provided invite code is valid
    pub async fn is_valid(pool: &PgPool, id: &String) -> Result<bool, TalliiError> {
        // get the invite code, if it exists
        let invite_code =
            sqlx::query_as::<_, InviteCode>("select * from invite_codes where id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?;

        // if invite code doesnt exist return false
        if invite_code.is_none() {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    /// Gets all invite codes
    /// TODO: Hide this behind the citadel
    pub async fn get_all(pool: &PgPool) -> Result<Vec<InviteCode>, TalliiError> {
        let all_invite_codes = sqlx::query_as::<_, InviteCode>("select * from invite_codes")
            .fetch_all(pool)
            .await?;

        Ok(all_invite_codes)
    }

    /// Creates invite codes for the specified amount
    /// Note, this query may not be the most performant query
    /// because it is not a bulk insert but that is fine.
    pub async fn create_many(pool: &PgPool, amount: i32) -> Result<(), TalliiError> {
        let mut new_codes_queries = Vec::new();

        // generate invite codes for the given amount
        for _ in 0..amount {
            let code = generate(6);

            new_codes_queries.push(
                sqlx::query("insert into invite_codes (id) values ($1)")
                    .bind(code)
                    .execute(pool),
            )
        }

        try_join_all(new_codes_queries).await?;

        Ok(())
    }
}

pub struct UsersTable;

impl UsersTable {
    /// Fetches a user with the provided email
    pub async fn get_by_email(pool: &PgPool, email: &String) -> Result<Option<User>, TalliiError> {
        let user_with_email = sqlx::query_as::<_, User>("select * from users where email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;

        Ok(user_with_email)
    }

    /// Fetches a user with the provided username
    pub async fn get_by_username(
        pool: &PgPool,
        username: &String,
    ) -> Result<Option<PublicUser>, TalliiError> {
        let user_with_username = sqlx::query_as::<_, PublicUser>(
            "select user_id, emoji, bg_color, username, bio from users where username = $1",
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        Ok(user_with_username)
    }

    /// Fetches a username that matches the provided username and user_id.
    pub async fn get_by_username_and_id(
        pool: &PgPool,
        user_id: &i32,
        username: &String,
    ) -> Result<Option<User>, TalliiError> {
        let user_with_id_and_username =
            sqlx::query_as::<_, User>("select * from users where user_id = $1 and username = $2")
                .bind(user_id)
                .bind(username)
                .fetch_optional(pool)
                .await?;

        Ok(user_with_id_and_username)
    }

    /// Fetches a user that holds the provided invite code
    pub async fn get_by_invite_code(
        pool: &PgPool,
        invite_code: &str,
    ) -> Result<Option<PublicUser>, TalliiError> {
        let user_with_invite_code =
            sqlx::query_as::<_, PublicUser>("select * from users where invite_code = $1")
                .bind(invite_code)
                .fetch_optional(pool)
                .await?;

        Ok(user_with_invite_code)
    }

    /// Fetches a user with the provided username
    pub async fn search_by_username(
        pool: &PgPool,
        params: &UserQuery,
    ) -> Result<Vec<PublicUser>, TalliiError> {
        let matching_users = sqlx::query_as::<_, PublicUser>(
            "select user_id, emoji, bg_color, username, bio from users where username like $1 limit 10",
        )
        .bind(format!("%{}%", &params.q))
        .fetch_all(pool)
        .await?;

        Ok(matching_users)
    }

    /// Creates a user
    pub async fn create(
        pool: &PgPool,
        new_user: &NewUser,
        crypto: &Crypto,
    ) -> Result<PublicUser, TalliiError> {
        // hash the password
        let hashed_password = crypto.hash_password(&new_user.password).await?;

        // generate default emoji, bg_color combo
        let avatar = Avatar::new();

        // create the user and return the public user
        let user = sqlx::query_as::<_, PublicUser>(
            "insert into users (email, password, emoji, bg_color, invite_code, username) values ($1, $2, $3, $4, $5, $6) returning user_id, emoji, bg_color, email, username, bio, verified",
        )
            .bind(&new_user.email)
            .bind(hashed_password)
            .bind(&avatar.emoji)
            .bind(&avatar.background)
            .bind(&new_user.invite_code)
            .bind(&new_user.username)
            .fetch_one(pool)
            .await?;

        Ok(user)
    }
}
