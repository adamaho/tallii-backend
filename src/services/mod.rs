use crate::errors::TalliiError;
use actix_web::HttpResponse;

pub mod auth;
pub mod friends;
pub mod groups;
pub mod users;

type TalliiResponse = Result<HttpResponse, TalliiError>;
