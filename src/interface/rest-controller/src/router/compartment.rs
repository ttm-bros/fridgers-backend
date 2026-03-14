use crate::handler;
use actix_web::web;
use fridgers_backend_use_case::Repository;

pub fn configure<R: Repository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/fridges/{fridge_id}/compartments",
        web::post().to(handler::compartment::create_compartment::<R>),
    )
    .route(
        "/v1/fridges/{fridge_id}/compartments/{compartment_id}",
        web::put().to(handler::compartment::update_compartment::<R>),
    )
    .route(
        "/v1/fridges/{fridge_id}/compartments/{compartment_id}",
        web::delete().to(handler::compartment::delete_compartment::<R>),
    );
}
