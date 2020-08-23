use actix_web::web;

pub mod auth;

/// Representation of a Service
pub trait Service {
    /// Defines the routes associated with the service
    fn define_routes(cfg: &mut web::ServiceConfig);
}
