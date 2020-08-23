use actix_web::{web, HttpResponse};

use crate::errors::TalliiError;
pub mod auth;

type TalliiResponse = Result<HttpResponse, TalliiError>;

/// Representation of a Service
pub trait Service {
    /// Defines the routes associated with the service
    fn define_routes(cfg: &mut web::ServiceConfig);
}
