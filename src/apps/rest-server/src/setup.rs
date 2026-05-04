use fridgers_backend_config::Config;
use fridgers_backend_use_case::Interactor;
use fridgers_backend_use_case::auth::JwtConfig;
use rdb_gateway::PostgresRepository;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
use tracing::Level;
use tracing_log::LogTracer;

/// ロガーの初期化
pub fn setup_logger(config: &Config) -> std::io::Result<()> {
    LogTracer::init().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(config.log.level.as_str()).unwrap())
        .try_init()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    Ok(())
}

/// 依存性の構築（DI Container）
pub async fn setup_dependencies(
    config: &Config,
) -> std::io::Result<(Arc<Interactor<PostgresRepository>>, JwtConfig)> {
    let pool = PgPool::connect(&config.db.database_url)
        .await
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to connect to database: {}", e),
            )
        })?;

    let jwt_config = JwtConfig {
        secret: config.auth.jwt_secret.clone(),
        expiry_hours: config.auth.jwt_expiry_hours,
    };

    let repository = PostgresRepository::new(pool);
    let interactor = Arc::new(Interactor::new(repository, jwt_config.clone()));
    Ok((interactor, jwt_config))
}
