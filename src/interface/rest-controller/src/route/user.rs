use actix_web::web;
use crate::handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(handler::user::register_user))
    );
}
