use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::errors::TalliiError;
use crate::models::user::User;

/// Represents the contents of a jwt
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub sub: i32,
    pub username: String,
    pub exp: i64,
}

impl Token {
    /// Creates an instance of a token from the provided user
    pub fn from_user(user: &User) -> Self {
        let now = Utc::now() + Duration::days(1);

        Token {
            sub: user.user_id.clone(),
            username: user.username.clone(),
            exp: now.timestamp(),
        }
    }

    /// Decodes the provided token to the Token struct
    pub fn verify(token: &str) -> Result<jsonwebtoken::TokenData<Token>, TalliiError> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET has not been defined");
        let token = decode::<Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token)
    }

    /// Encodes the provided token struct to a string
    pub fn generate(&self) -> String {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET has not been defined");
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }
}
