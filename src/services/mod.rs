use actix_web::{web, HttpResponse};

use crate::errors::Test;
pub mod auth;

type TalliiResponse = Result<HttpResponse, Test>;

/// Representation of a Service
pub trait Service {
    /// Defines the routes associated with the service
    fn define_routes(cfg: &mut web::ServiceConfig);
}
