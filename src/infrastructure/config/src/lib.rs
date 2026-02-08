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
    pub url: String,
    pub port: u16,
    pub database_url: String,
}

#[derive(Deserialize)]
struct DbEnvConfig {
    url: String,
    port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv()?;

        let log = prefixed("LOG_").from_env::<LogConfig>()?;
        let server = prefixed("SERVER_").from_env::<ServerConfig>()?;
        let db_env = prefixed("DB_").from_env::<DbEnvConfig>()?;
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|e| Error::Load(format!("DATABASE_URL: {}", e)))?;

        let db = DbConfig {
            url: db_env.url,
            port: db_env.port,
            database_url,
        };

        Ok(Self {
            log,
            server,
            db,
            _hide_default_constructor: PhantomData,
        })
    }
}
