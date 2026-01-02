use actix_web::{App, HttpServer, dev::Service, middleware, web};
use std::str::FromStr;
use tracing::{Level, info_span};
use tracing_log::LogTracer;

async fn health_check() -> &'static str {
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = fridgers_backend_config::Config::from_env().unwrap();

    let _ = LogTracer::init();
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::from_str(config.log.level.as_str()).unwrap())
        .try_init();

    HttpServer::new(|| {
        App::new()
            // トレース用のスパンを追加
            .wrap_fn(|req, srv| {
                let span = info_span!(
                    "http_request",
                    method = %req.method(),
                    path = %req.path()
                );
                let fut = srv.call(req);
                async move {
                    let _enter = span.enter();
                    let res = fut.await?;
                    Ok(res)
                }
            })
            // アクセスログの追加
            .wrap(middleware::Logger::default())
            // ヘルスチェックエンドポイント
            .route("/liveness", web::get().to(health_check))
            // ユーザー関連のエンドポイントを設定
            .configure(rest_controller::configure_users)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
