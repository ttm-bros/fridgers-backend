use actix_web::{App, HttpServer, dev::Service, web};
use fridgers_backend_rest_server::{app, setup};
use tracing::info_span;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 設定の読み込み
    let config = fridgers_backend_config::Config::from_env()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;

    // ロガーの初期化
    setup::setup_logger(&config)?;

    // 依存性の構築
    let interactor = setup::setup_dependencies();

    // HTTPサーバーの起動
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
            .wrap(app::logger())
            // エンドポイントの設定
            .configure(app::configure_routes)
    })
    .bind((config.server.url.as_str(), config.server.port))?
    .run()
    .await
}
