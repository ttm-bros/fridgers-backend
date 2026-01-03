use actix_web::get;

#[get("/liveness")]
pub async fn health_check() -> &'static str {
    "OK"
}
