use actix_web::web;
use crate::handler;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(handler::user::register_user);
}
