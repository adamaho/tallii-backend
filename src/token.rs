use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::errors::TalliiError;
use crate::models::user::User;

/// Represents the contents of a jwt
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub sub: i32,
    pub username: String,
    pub iat: u64,
    pub exp: u64,
}

impl Token {
    /// Creates an instance of a token from the provided user
    pub fn from_user(user: &User) -> Self {
        let start = SystemTime::now();
        let iat = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let exp = iat + Duration::from_secs(60 * 60 * 24 * 7).as_secs();

        Token {
            sub: user.user_id.clone(),
            username: user.username.clone(),
            iat,
            exp,
        }
    }

    /// Decodes the provided token to the Token struct
    pub fn decode(token: &str) -> Result<jsonwebtoken::TokenData<Token>, TalliiError> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET has not been defined");
        let token = decode::<Token>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token)
    }

    /// Encodes the provided token struct to a string
    pub fn encode(&self) -> String {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET has not been defined");
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }
}