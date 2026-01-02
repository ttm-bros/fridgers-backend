use actix_web::{App, HttpServer, dev::Service, middleware, web};
use fridgers_backend_use_case::interactor::FridgersRestInteractor;
use rdb_gateway::InMemoryUserRepository;
use std::str::FromStr;
use std::sync::Arc;
use tracing::{Level, info_span};
use tracing_log::LogTracer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = fridgers_backend_config::Config::from_env().unwrap();

    let _ = LogTracer::init();
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::from_str(config.log.level.as_str()).unwrap())
        .try_init();

    // 依存性の構築
    let repository = InMemoryUserRepository::new();
    let interactor = Arc::new(FridgersRestInteractor::new(Box::new(repository)));

    HttpServer::new(move || {
        App::new()
            // DIコンテナの登録
            .app_data(web::Data::new(interactor.clone()))
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
            // エンドポイントの設定
            .configure(rest_controller::configure_health)
            .configure(rest_controller::configure_users)
    })
    .bind((config.server.url.as_str(), config.server.port))?
    .run()
    .await
}
