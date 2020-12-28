use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::errors::TalliiError;

pub mod auth;
pub mod events;
pub mod friends;
pub mod users;

type TalliiResponse = Result<HttpResponse, TalliiError>;

#[derive(Serialize, Deserialize)]
pub struct SuccessResponse {
    code: String,
    message: String,
}
