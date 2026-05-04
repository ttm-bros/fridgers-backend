use crate::error::Result;
use crate::extractor::AuthenticatedUser;
use crate::schema::item::create::{self as create_schema};
use crate::schema::item::update::{self as update_schema};
use actix_web::{HttpResponse, web};
use fridgers_backend_domain::compartment::CompartmentId;
use fridgers_backend_domain::item::{ItemId, ItemName, ItemUnit};
use fridgers_backend_use_case::{
    self as use_case, Interactor, Repository, dto::item::create::CreateItemRequest,
    dto::item::update::UpdateItemRequest,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_item<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    user: AuthenticatedUser,
    path: web::Path<(String, String)>,
    req: web::Json<create_schema::CreateItemRequest>,
) -> Result<HttpResponse> {
    let (_fridge_id_str, compartment_id_str) = path.into_inner();

    let compartment_uuid = Uuid::parse_str(&compartment_id_str)
        .map_err(|e| use_case::Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
    let compartment_id = CompartmentId::from(compartment_uuid);

    let item_id = ItemId::new();
    let name = ItemName::try_from(req.name.clone()).map_err(use_case::Error::from)?;
    let unit = ItemUnit::try_from(req.unit.clone()).map_err(use_case::Error::from)?;

    let use_case_request = CreateItemRequest::new(
        item_id,
        compartment_id,
        name,
        req.quantity,
        unit,
        req.expires_at,
    );
    let item = interactor
        .handle_create_item(&user.user_id, use_case_request)
        .await?;

    let response = create_schema::CreateItemResponse::from(item);
    Ok(HttpResponse::Created().json(response))
}

pub async fn update_item<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    user: AuthenticatedUser,
    path: web::Path<(String, String, String)>,
    req: web::Json<update_schema::UpdateItemRequest>,
) -> Result<HttpResponse> {
    let (_fridge_id_str, compartment_id_str, item_id_str) = path.into_inner();

    let item_uuid = Uuid::parse_str(&item_id_str)
        .map_err(|e| use_case::Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
    let item_id = ItemId::from(item_uuid);

    let name = ItemName::try_from(req.name.clone()).map_err(use_case::Error::from)?;
    let unit = ItemUnit::try_from(req.unit.clone()).map_err(use_case::Error::from)?;

    let use_case_request =
        UpdateItemRequest::new(item_id, name, req.quantity, unit, req.expires_at);
    let item = interactor
        .handle_update_item(&user.user_id, &compartment_id_str, use_case_request)
        .await?;

    let response = update_schema::UpdateItemResponse::from(item);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_item<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    user: AuthenticatedUser,
    path: web::Path<(String, String, String)>,
) -> Result<HttpResponse> {
    let (_fridge_id_str, compartment_id_str, item_id_str) = path.into_inner();
    interactor
        .handle_delete_item(&user.user_id, &compartment_id_str, &item_id_str)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
