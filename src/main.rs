use actix_web::{middleware::Logger, web, App, HttpServer};
use tracing::{info, instrument};

mod config;
mod crypto;
mod errors;
mod models;
mod repositories;
mod services;

// bring in the service trait
use crate::services::{auth::Auth, Service};

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
            .wrap(Logger::default())
            .data(pool.clone())
            .data(crypto.clone())
            .service(web::scope("/auth").configure(Auth::define_routes))
    })
    .bind(format!("{}:{}", config.hostname, config.port))?
    .run()
    .await?;

    Ok(())
}
