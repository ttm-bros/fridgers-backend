use actix_web::{App, test, web};
use fridgers_backend_rest_server::app;
use fridgers_backend_use_case::Interactor;
use fridgers_backend_use_case::auth::JwtConfig;
use rdb_gateway::PostgresRepository;
use sqlx::PgPool;
use std::sync::Arc;

/// テスト用のDBプールを作成する
async fn create_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://fridgers:fridgers_password@localhost:5432/fridgers".to_string()
    });

    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

/// テスト用のActix-webアプリを構築する
pub async fn create_test_app() -> (
    impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
    PgPool,
) {
    let pool = create_test_pool().await;
    let repository = PostgresRepository::new(pool.clone());
    let jwt_config = JwtConfig {
        secret: "test-secret-key".to_string(),
        expiry_hours: 24,
    };
    let interactor = Arc::new(Interactor::new(repository, jwt_config.clone()));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(interactor))
            .app_data(web::Data::new(jwt_config))
            .configure(app::configure_routes::<PostgresRepository>),
    )
    .await;

    (app, pool)
}

/// テスト後にfridges→usersの順でクリーンアップする（FK制約のため順序が重要）
pub async fn cleanup_users(pool: &PgPool) {
    sqlx::query("DELETE FROM fridges")
        .execute(pool)
        .await
        .expect("Failed to cleanup fridges table");
    sqlx::query("DELETE FROM users")
        .execute(pool)
        .await
        .expect("Failed to cleanup users table");
}
