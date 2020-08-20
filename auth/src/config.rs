use dotenv::dotenv;
use serde::Deserialize;
use tracing::{info, instrument};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub hostname: String,
    pub port: i16,
    pub database_url: String,
    pub secret: String,
    pub jwt_secret: String,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();

        info!("Loading config");

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
    }
}
