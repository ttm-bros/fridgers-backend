use dotenvy::dotenv;
use serde::Deserialize;
use std::marker::PhantomData;

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

        // SERVER_ プレフィックスの環境変数を ServerConfig に
        let server = envy::prefixed("SERVER_").from_env::<ServerConfig>()?;

        // DB_ プレフィックスの環境変数を DbConfig に
        let db = envy::prefixed("DB_").from_env::<DbConfig>()?;

        Ok(Self {
            server,
            db,
            _hide_default_constructor: PhantomData,
        })
    }
}
