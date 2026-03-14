use crate::handler;
use actix_web::web;
use fridgers_backend_use_case::Repository;

pub fn configure<R: Repository + 'static>(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/v1/fridges/{fridge_id}/compartments/{compartment_id}/items",
        web::post().to(handler::item::create_item::<R>),
    )
    .route(
        "/v1/fridges/{fridge_id}/compartments/{compartment_id}/items/{item_id}",
        web::put().to(handler::item::update_item::<R>),
    )
    .route(
        "/v1/fridges/{fridge_id}/compartments/{compartment_id}/items/{item_id}",
        web::delete().to(handler::item::delete_item::<R>),
    );
}
