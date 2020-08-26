use std::sync::Arc;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::errors::TalliiError;

/// Represents the contents of a jwt
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub username: String,
    pub exp: i64,
}

#[derive(Debug, Clone)]
pub struct Crypto {
    pub hash_secret: Arc<String>,
    pub jwt_secret: Arc<String>
}

impl Crypto {

    /// Decodes the provided token to the Token struct
    pub fn verify_jwt(token: &str) -> Result<jsonwebtoken::TokenData<Claims>, TalliiError> {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET has not been defined");
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )?;

        Ok(token)
    }

    /// Encodes the provided token struct to a string
    pub fn generate_jwt(&self, user_id: i32, username: String) -> String {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET has not been defined");

        let now = Utc::now() + Duration::days(1); // Expires in 1 day
        let claims =  Claims {
            sub: user_id,
            username,
            exp: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap()
    }

    // Hashes the provided password
    // pub fn hash_password(password: String) -> Result<String, TalliiError> {

    // }
}