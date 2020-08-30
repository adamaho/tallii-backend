use std::ops::Deref;

use actix_web::{web, HttpResponse, HttpRequest, FromRequest};
use actix_web::dev::Payload;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::future::BoxFuture;
use serde::Deserialize;
use sqlx::PgPool;

use crate::crypto::{Crypto, Claims};
use crate::errors::TalliiError;
use crate::repositories::user::UserRepository;

pub mod auth;

type TalliiResponse = Result<HttpResponse, TalliiError>;

/// Representation of a Service
pub trait Service {
    /// Defines the routes associated with the service
    fn define_routes(cfg: &mut web::ServiceConfig);
}

#[derive(Debug, Deserialize)]
pub struct AuthenticatedUser {
    user_id: i32,
    username: String
}

impl FromRequest for AuthenticatedUser {
    type Error = TalliiError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // get the auth bearer header from the bearer extractor
        let bearer = BearerAuth::from_request(req, payload).into_inner();

        // get the pool
        let pool = web::Data::<PgPool>::from_request(req, payload).into_inner();

        // get the crypto service
        let crypto = web::Data::<Crypto>::from_request(req, payload).into_inner();

        // match on a tuple of the result of the above
        match (bearer, pool, crypto) {
            (Ok(b), Ok(p), Ok(c)) => {
                let future = async move {
                    // get the claims
                    let claims: Claims = c.verify_jwt(b.token().to_string())
                        .await
                        .map(|data| data )
                        .map_err(|_err| { TalliiError::UNAUTHORIZED.default() });

                    // get an instance of the user repo
                    let user_repo = UserRepository::new(pool.deref().clone());

                    // check to make sure the provided username and user_id combo is valid
                    user_repo.get_by_username_and_id(&claims.sub, &claims.username).await?
                        .ok_or_else(|| {
                            TalliiError::UNAUTHORIZED.default()
                        });

                    Ok()
                };

                Box::pin(future)
            }
        }
    }
}
