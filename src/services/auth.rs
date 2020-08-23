use std::ops::Deref;

use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use super::TalliiResponse;
use crate::db::invite_code::InviteCodeRepository;
use crate::models::invite_code::InviteCode;
use crate::services::Service;

/// Checks the validity of the invite code
pub async fn check_invite_code(
    pool: web::Data<PgPool>,
    web::Json(code): web::Json<InviteCode>,
) -> TalliiResponse {

    // get the invite code repository
    let repository = InviteCodeRepository::new(pool.deref().clone());

    // execute the query
    match repository.get_by_id(&code.id).await? {
        // invite codes match so return 200
        Some(_) => Ok(HttpResponse::Ok().finish()),

        // invite codes match so return 200
        None => Ok(HttpResponse::UnprocessableEntity().finish())
    }
}

/// Logs the user in if the provided credentials are correct
pub async fn login() -> &'static str {  
    "Login"
}

/// Signs a user up with the provided credentials
pub async fn signup() -> &'static str {
    "Signup"
}

pub struct Auth;

impl Service for Auth {
    fn define_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/invite-codes").route(web::get().to(check_invite_code)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/signup").route(web::post().to(signup)));
    }
}
