use crate::error::Result;
use crate::schema::fridge::create::{self as create_schema};
use crate::schema::fridge::get::{self as get_schema};
use actix_web::{HttpResponse, web};
use fridgers_backend_domain::fridge::{FridgeId, FridgeName};
use fridgers_backend_domain::user::UserId;
use fridgers_backend_use_case::{self as use_case, Interactor, Repository, dto::fridge::create as dto};
use std::sync::Arc;

pub async fn create_fridge<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    req: web::Json<create_schema::CreateFridgeRequest>,
) -> Result<HttpResponse> {
    let fridge_id = FridgeId::new();
    let fridge_name = FridgeName::try_from(req.name.clone()).map_err(use_case::Error::from)?;
    let owner_user_id = UserId::try_from(req.owner_user_id.clone()).map_err(use_case::Error::from)?;

    let use_case_request = dto::CreateFridgeRequest::new(fridge_id, fridge_name, owner_user_id);

    let fridge = interactor.handle_create_fridge(use_case_request).await?;

    let response = create_schema::CreateFridgeResponse::from(fridge);
    Ok(HttpResponse::Created().json(response))
}

pub async fn get_fridge<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let fridge_id = path.into_inner();
    let fridge_with_compartments = interactor.handle_get_fridge(&fridge_id).await?;
    let response = get_schema::GetFridgeResponse::from(fridge_with_compartments);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_fridge<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let fridge_id = path.into_inner();
    interactor.handle_delete_fridge(&fridge_id).await?;
    Ok(HttpResponse::NoContent().finish())
}
