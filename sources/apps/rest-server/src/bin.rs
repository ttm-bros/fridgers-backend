use actix_web::{App, HttpRequest, HttpServer, Responder, dev::Service, web};
use std::str::FromStr;
use tracing::{Level, info_span};
use tracing_log::LogTracer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = fridgers_backend_config::Config::from_env().unwrap();

    LogTracer::init().ok();
    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(config.log.level.as_str()).unwrap())
        .init();

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
            .wrap(actix_web::middleware::Logger::default())
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
