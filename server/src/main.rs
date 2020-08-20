use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use tracing::{info, instrument};
mod config;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

#[actix_rt::main]
#[instrument]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env().expect("failed to load environment");

    let pool = config
        .setup_database()
        .await
        .expect("failed to create database pool");

    info!(
        "Starting server at http://{}:{}",
        config.hostname, config.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
    })
    .bind(format!("{}:{}", config.hostname, config.port))?
    .run()
    .await?;

    Ok(())
}
