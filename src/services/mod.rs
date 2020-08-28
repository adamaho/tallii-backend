use std::ops::Deref;

use actix_web::{web, HttpResponse, HttpRequest, FromRequest};
use actix_web::dev::Payload;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::future::BoxFuture;
use serde::Deserialize;
use sqlx::PgPool;

use crate::crypto::Crypto;
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


        match (bearer, pool, crypto) {
            (Ok(bearer), Ok(pool), Ok(crypto)) => {
                //
            }
        }

    }
}
