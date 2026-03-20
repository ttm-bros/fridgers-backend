use actix_web::{middleware, web};
use fridgers_backend_use_case::Repository;

/// エンドポイントの設定
pub fn configure_routes<R: Repository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.configure(rest_controller::configure_health)
        .configure(rest_controller::configure_users::<R>)
        .configure(rest_controller::configure_fridges::<R>)
        .configure(rest_controller::configure_compartments::<R>)
        .configure(rest_controller::configure_items::<R>)
        .configure(rest_controller::configure_auth::<R>);
}

/// アクセスログのミドルウェアを取得
pub fn logger() -> middleware::Logger {
    middleware::Logger::default()
}
