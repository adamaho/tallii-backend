use std::sync::Arc;
use std::env;

use argonautica::{Hasher, Verifier};
use chrono::{Duration, Utc};
use futures::compat::Future01CompatExt;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

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
    pub fn verify_jwt(&self, token: &str) -> Result<jsonwebtoken::TokenData<Claims>, TalliiError> {
        let token = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.jwt_secret.clone().as_bytes()),
            &Validation::default(),
        )?;

        Ok(token)
    }

    /// Encodes the provided token struct to a string
    pub fn generate_jwt(&self, user_id: i32, username: String) -> String {
        let now = Utc::now() + Duration::days(1); // Expires in 1 day
        let claims =  Claims {
            sub: user_id,
            username,
            exp: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.clone().as_bytes()),
        )
        .unwrap()
    }

    // Hashes the provided password
    pub async fn hash_password(&self, password: &str) -> Result<String, TalliiError> {

        match Hasher::default()
            .with_secret_key(&*self.hash_secret)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await {
                Ok(hashed_password) => Ok(hashed_password),
                Err(_) => {
                    info!("Failed to hash password.");
                    Err(TalliiError::InternalServerError)
                }
            }

    }

    pub async fn verify_password(&self, password: &str, hashed_password: &str) -> Result<bool, TalliiError> {
        match Verifier::default()
            .with_hash(hashed_password)
            .with_password(password)
            .with_secret_key(&*self.hash_secret)
            .verify_non_blocking()
            .compat()
            .await {
                Ok(is_valid) => Ok(is_valid),
                Err(_) => {
                    info!("Failed to verify password.");
                    Err(TalliiError::Unauthorized)
                }
            }
    }
}