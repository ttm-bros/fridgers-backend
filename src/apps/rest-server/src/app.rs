use actix_web::{middleware, web};

/// エンドポイントの設定
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.configure(rest_controller::configure_health)
        .configure(rest_controller::configure_users);
}

/// アクセスログのミドルウェアを取得
pub fn logger() -> middleware::Logger {
    middleware::Logger::default()
}
