use dotenvy::dotenv;
use serde::Deserialize;
use std::marker::PhantomData;
use envy::prefixed;

#[derive(Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DbConfig,
    _hide_default_constructor: PhantomData<()>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub url: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DbConfig {
    pub url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().expect(".env file not found");

        let server = prefixed("SERVER_").from_env::<ServerConfig>()?;
        let db = prefixed("DB_").from_env::<DbConfig>()?;

        Ok(Self {
            server,
            db,
            _hide_default_constructor: PhantomData,
        })
    }
}
