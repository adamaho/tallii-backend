use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use tracing::{info, instrument};

mod config;
mod crypto;
mod errors;
mod routes;
mod services;

use crate::routes::define_routes;

#[actix_rt::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    // get the config from the environment
    let config = config::Config::from_env().expect("failed to load environment");

    // create database pool of connections
    let pool = config
        .setup_database()
        .await
        .expect("failed to create database pool");

    // get instance of crypto to be used for hashing
    let crypto = config.get_crypto();

    info!(
        "starting server at http://{}:{}",
        config.hostname, config.port
    );

    HttpServer::new(move || {
        App::new()
            // .wrap(
            //     Cors::new()
            //         .allowed_origin("http://localhost:1234")
            //         .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            //         .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //         .allowed_header(header::CONTENT_TYPE)
            //         .max_age(3600)
            //         .finish(),
            // )
            .wrap(Logger::default())
            .data(pool.clone())
            .data(crypto.clone())
            .service(web::scope("/api/v1").configure(define_routes))
            .route(
                "/",
                web::get().to(|| web::HttpResponse::Ok().json("Healthy")),
            )
    })
    .bind(format!("{}:{}", config.hostname, config.port))?
    .run()
    .await?;

    Ok(())
}
