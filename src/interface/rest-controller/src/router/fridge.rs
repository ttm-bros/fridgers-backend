use crate::handler;
use actix_web::web;
use fridgers_backend_use_case::Repository;

pub fn configure<R: Repository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/fridges",
        web::get().to(handler::fridge::list_fridges::<R>),
    )
    .route(
        "/v1/fridges",
        web::post().to(handler::fridge::create_fridge::<R>),
    )
    .route(
        "/v1/fridges/{fridge_id}",
        web::get().to(handler::fridge::get_fridge::<R>),
    )
    .route(
        "/v1/fridges/{fridge_id}",
        web::delete().to(handler::fridge::delete_fridge::<R>),
    );
}
