use actix_web::{HttpResponse, web};
use sqlx::PgPool;

use crate::services::TalliiResponse;

pub async fn get_me_wagers(pool: web::Data<PgPool>,) -> TalliiResponse {
    Ok(HttpResponse::Ok().json(""))
}

pub async fn get_me_invitations(pool: web::Data<PgPool>,) -> TalliiResponse {
    Ok(HttpResponse::Ok().json(""))
}

pub async fn get_users_wagers(pool: web::Data<PgPool>,) -> TalliiResponse {
    Ok(HttpResponse::Ok().json(""))
}

pub async fn create_wager(pool: web::Data<PgPool>,) -> TalliiResponse {
    Ok(HttpResponse::Ok().json(""))
}

pub async fn get_wager(pool: web::Data<PgPool>,) -> TalliiResponse {
    Ok(HttpResponse::Ok().json(""))
}

pub async fn update_wager_currency(pool: web::Data<PgPool>,) -> TalliiResponse {
    Ok(HttpResponse::Ok().json(""))
}

pub async fn decline_wager_invite(pool: web::Data<PgPool>,) -> TalliiResponse {
    Ok(HttpResponse::Ok().json(""))
}