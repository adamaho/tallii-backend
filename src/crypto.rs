use std::sync::Arc;

use actix_web::web;
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
    pub jwt_secret: Arc<String>,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

impl Crypto {
    /// Decodes the provided token to the Token struct
    pub async fn verify_jwt(
        &self,
        token: String,
    ) -> Result<jsonwebtoken::TokenData<Claims>, TalliiError> {
        let jwt_secret = self.jwt_secret.clone();

        web::block(move || {
            decode::<Claims>(
                &token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::default(),
            )
        })
        .await
        .map_err(|_err| TalliiError::UNAUTHORIZED.default())
    }

    /// Encodes the provided token struct to a string
    pub async fn generate_jwt(
        &self,
        user_id: i32,
        username: String,
    ) -> Result<String, TalliiError> {
        let jwt_secret = self.jwt_secret.clone();

        web::block(move || {
            let now = Utc::now() + Duration::days(365); // Expires in 1 year
            let claims = Claims {
                sub: user_id,
                username,
                exp: now.timestamp(),
            };

            encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(jwt_secret.as_bytes()),
            )
        })
        .await
        .map_err(|_err| {
            TalliiError::INTERNAL_SERVER_ERROR.message("Failed to create jwt token".to_string())
        })
    }

    // Hashes the provided password
    pub async fn hash_password(&self, password: &str) -> Result<String, TalliiError> {
        match Hasher::default()
            .with_secret_key(&*self.hash_secret)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
        {
            Ok(hashed_password) => Ok(hashed_password),
            Err(_) => {
                info!("Failed to hash password.");
                Err(TalliiError::INTERNAL_SERVER_ERROR.default())
            }
        }
    }

    pub async fn verify_password(
        &self,
        password: &str,
        hashed_password: &str,
    ) -> Result<bool, TalliiError> {
        match Verifier::default()
            .with_hash(hashed_password)
            .with_password(password)
            .with_secret_key(&*self.hash_secret)
            .verify_non_blocking()
            .compat()
            .await
        {
            Ok(is_valid) => Ok(is_valid),
            Err(_) => {
                info!("Failed to verify password.");
                Err(TalliiError::INTERNAL_SERVER_ERROR.default())
            }
        }
    }
}
