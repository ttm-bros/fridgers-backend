mod error;
pub use error::{Error, Result};

use dotenvy::dotenv;
use envy::prefixed;
use serde::Deserialize;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Config {
    pub log: LogConfig,
    pub server: ServerConfig,
    pub db: DbConfig,
    _hide_default_constructor: PhantomData<()>,
}

#[derive(Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub url: String,
    pub port: u16,
}

#[derive(Debug)]
pub struct DbConfig {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv()?;

        let log = prefixed("LOG_").from_env::<LogConfig>()?;
        let server = prefixed("SERVER_").from_env::<ServerConfig>()?;
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|e| Error::Load(format!("DATABASE_URL: {}", e)))?;

        let db = DbConfig { database_url };

        Ok(Self {
            log,
            server,
            db,
            _hide_default_constructor: PhantomData,
        })
    }
}
