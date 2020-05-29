//! Inject dotenv and env variables into the Config struct
//!
//! The envy crate injects environment variables into a struct.
//!
//! dotenv allows environment variables to be augmented/overwriten by a
//! .env file.
//!
//! This file throws the Config struct into a CONFIG lazy_static to avoid
//! multiple processing.

use crate::database::connection::DatabaseConnection;
use dotenv::dotenv;
use serde::Deserialize;

pub mod tls;


#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub auth_salt: String,
    pub database: DatabaseConnection,
    pub database_url: String,
    pub jwt_expiration: i64,
    pub jwt_key: String,
    pub redis_url: String,
    pub rust_log: String,
    pub server: String,
    pub port: String,
    pub secure_port: String,
    pub session_key: String,
    pub session_name: String,
    pub session_secure: bool,
    pub session_samesite: String,
    pub session_timeout: i64,
    pub actix_ssl_cert_file: String,
    pub actix_ssl_key_file: String
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    dotenv().ok();
    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_a_config() {
        let config = get_config();
        assert_ne!(config.server, "".to_string());
    }

    #[test]
    fn it_gets_a_config_from_the_lazy_static() {
        let config = &CONFIG;
        assert_ne!(config.server, "".to_string());
    }
}
