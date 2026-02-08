use actix_web::{App, test, web};
use fridgers_backend_rest_server::app;
use fridgers_backend_use_case::Interactor;
use rdb_gateway::PostgresRepository;
use sqlx::PgPool;
use std::sync::Arc;

/// テスト用のDBプールを作成する
async fn create_test_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://fridgers:fridgers_password@localhost:5432/fridgers".to_string());

    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

/// テスト用のActix-webアプリを構築する
pub async fn create_test_app() -> (impl actix_web::dev::Service<actix_http::Request, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error>, PgPool) {
    let pool = create_test_pool().await;
    let repository = PostgresRepository::new(pool.clone());
    let interactor = Arc::new(Interactor::new(repository));

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(interactor))
            .configure(app::configure_routes::<PostgresRepository>),
    )
    .await;

    (app, pool)
}

/// テスト後にusersテーブルをクリーンアップする
pub async fn cleanup_users(pool: &PgPool) {
    sqlx::query("DELETE FROM users")
        .execute(pool)
        .await
        .expect("Failed to cleanup users table");
}
