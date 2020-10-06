use actix_web::HttpResponse;
use crate::errors::TalliiError;

pub mod auth;
pub mod friends;
pub mod groups;
pub mod users;

type TalliiResponse = Result<HttpResponse, TalliiError>;
