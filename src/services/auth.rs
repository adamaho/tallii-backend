use actix_web::web;
use sqlx::PgPool;

use crate::models::invite_code::InviteCode;
use crate::services::Service;

/// Checks the validity of the invite code
pub async fn check_invite_code(
    web::Json(invite_code): web::Json<InviteCode>,
    pool: web::Data<PgPool>,
) -> &'static str {
    println!("{:?}", invite_code);
    "Checked the Key"
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
        cfg.service(web::resource("/invite_codes").route(web::post().to(check_invite_code)))
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/signup").route(web::post().to(signup)));
    }
}
