use fridgers_backend_config::Config;
use fridgers_backend_use_case::Interactor;
use rdb_gateway::InMemoryUserRepository;
use std::str::FromStr;
use std::sync::Arc;
use tracing::Level;
use tracing_log::LogTracer;

/// ロガーの初期化
pub fn setup_logger(config: &Config) -> std::io::Result<()> {
    LogTracer::init()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(config.log.level.as_str()).unwrap())
        .try_init()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}

/// 依存性の構築（DI Container）
pub fn setup_dependencies() -> Arc<Interactor> {
    let repository = InMemoryUserRepository::new();
    Arc::new(Interactor::new(Box::new(repository)))
}
