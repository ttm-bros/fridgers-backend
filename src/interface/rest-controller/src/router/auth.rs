use actix_web::web;
use crate::handler;
use fridgers_backend_use_case::Repository;

pub fn configure<R: Repository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.route("/v1/auth/login", web::post().to(handler::auth::login::<R>));
}
