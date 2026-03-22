use crate::error::Result;
use crate::schema::compartment::create::{self as create_schema};
use crate::schema::compartment::update::{self as update_schema};
use actix_web::{HttpResponse, web};
use fridgers_backend_domain::compartment::{CompartmentId, CompartmentName};
use fridgers_backend_domain::fridge::FridgeId;
use fridgers_backend_use_case::{
    self as use_case, Interactor, Repository,
    dto::compartment::create::CreateCompartmentRequest,
    dto::compartment::update::UpdateCompartmentRequest,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_compartment<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    path: web::Path<String>,
    req: web::Json<create_schema::CreateCompartmentRequest>,
) -> Result<HttpResponse> {
    let fridge_id_str = path.into_inner();
    let fridge_uuid = Uuid::parse_str(&fridge_id_str)
        .map_err(|e| use_case::Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
    let fridge_id = FridgeId::from(fridge_uuid);

    let compartment_id = CompartmentId::new();
    let name = CompartmentName::try_from(req.name.clone()).map_err(use_case::Error::from)?;

    let use_case_request = CreateCompartmentRequest::new(compartment_id, fridge_id, name);
    let compartment = interactor
        .handle_create_compartment(use_case_request)
        .await?;

    let response = create_schema::CreateCompartmentResponse::from(compartment);
    Ok(HttpResponse::Created().json(response))
}

pub async fn update_compartment<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    path: web::Path<(String, String)>,
    req: web::Json<update_schema::UpdateCompartmentRequest>,
) -> Result<HttpResponse> {
    let (fridge_id_str, compartment_id_str) = path.into_inner();

    let fridge_uuid = Uuid::parse_str(&fridge_id_str)
        .map_err(|e| use_case::Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
    let fridge_id = FridgeId::from(fridge_uuid);

    let compartment_uuid = Uuid::parse_str(&compartment_id_str)
        .map_err(|e| use_case::Error::InvalidArgument(format!("Invalid UUID format: {}", e)))?;
    let compartment_id = CompartmentId::from(compartment_uuid);

    let name = CompartmentName::try_from(req.name.clone()).map_err(use_case::Error::from)?;

    let use_case_request = UpdateCompartmentRequest::new(compartment_id, fridge_id, name);
    let compartment = interactor
        .handle_update_compartment(use_case_request)
        .await?;

    let response = update_schema::UpdateCompartmentResponse::from(compartment);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_compartment<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse> {
    let (fridge_id_str, compartment_id_str) = path.into_inner();
    interactor
        .handle_delete_compartment(&fridge_id_str, &compartment_id_str)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
