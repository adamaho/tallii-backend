use std::ops::Deref;

use actix_web::dev::Payload;
use actix_web::{web, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::future::{ready, BoxFuture};
use sqlx::PgPool;

use crate::crypto::Crypto;
use crate::errors::TalliiError;
use crate::services::users::db::UserRepository;

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = TalliiError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // get the users bearer header from the bearer extractor
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
                    let token = c
                        .verify_jwt(b.token().to_string())
                        .await
                        .map_err(|_err| TalliiError::UNAUTHORIZED.default())?;

                    // get an instance of the user repo
                    let user_repo = UserRepository::new(p.deref().clone());

                    // check to make sure the provided username and user_id combo is valid
                    user_repo
                        .get_by_username_and_id(&token.claims.sub, &token.claims.username)
                        .await?
                        .ok_or_else(|| TalliiError::UNAUTHORIZED.default())?;

                    // return the authenticated user
                    Ok(AuthenticatedUser {
                        user_id: token.claims.sub,
                        username: token.claims.username,
                    })
                };

                Box::pin(future)
            }
            _ => {
                // create the error by immediately returning from the future
                let error = ready(Err(TalliiError::UNAUTHORIZED.default()));
                Box::pin(error)
            }
        }
    }
}
