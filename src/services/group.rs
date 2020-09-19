use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use super::{Service, TalliiResponse};

use super::AuthenticatedUser;
use crate::errors::TalliiError;

/// Creates a new group
pub async fn create_group(
    pool: web::Data<PgPool>,
) -> TalliiResponse {
    Ok(HttpResponse::Ok().finish())
}

/// Creates a new group
pub async fn get_groups(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser
) -> TalliiResponse {
    println!("made it past");
    Ok(HttpResponse::Ok().finish())
}

/// Creates a new group
pub async fn update_group(
    pool: web::Data<PgPool>,
) -> TalliiResponse {
    Ok(HttpResponse::Ok().finish())
}

/// Creates a new group
pub async fn delete_group(
    pool: web::Data<PgPool>,
    user: AuthenticatedUser
) -> TalliiResponse {
    Ok(HttpResponse::Ok().finish())
}

pub struct GroupService;

impl Service for GroupService {
    fn define_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/")
                .route(web::post().to(create_group))
                .route(web::get().to(get_groups)),
        )
        .service(
            web::resource("/{group_id}")
                .route(web::put().to(update_group))
                .route(web::delete().to(delete_group))
        );
    }
}
