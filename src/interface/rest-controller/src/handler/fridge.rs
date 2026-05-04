use crate::error::Result;
use crate::extractor::AuthenticatedUser;
use crate::schema::fridge::create::{self as create_schema};
use crate::schema::fridge::get::{self as get_schema};
use crate::schema::fridge::list::{self as list_schema};
use actix_web::{HttpResponse, web};
use fridgers_backend_domain::fridge::{FridgeId, FridgeName};
use fridgers_backend_domain::user::UserId;
use fridgers_backend_use_case::{
    self as use_case, Interactor, Repository, dto::fridge::create as dto,
};
use std::sync::Arc;

pub async fn create_fridge<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    user: AuthenticatedUser,
    req: web::Json<create_schema::CreateFridgeRequest>,
) -> Result<HttpResponse> {
    let fridge_id = FridgeId::new();
    let fridge_name = FridgeName::try_from(req.name.clone()).map_err(use_case::Error::from)?;
    let owner_user_id = UserId::try_from(user.user_id).map_err(use_case::Error::from)?;

    let use_case_request = dto::CreateFridgeRequest::new(fridge_id, fridge_name, owner_user_id);

    let fridge = interactor.handle_create_fridge(use_case_request).await?;

    let response = create_schema::CreateFridgeResponse::from(fridge);
    Ok(HttpResponse::Created().json(response))
}

pub async fn get_fridge<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let fridge_id = path.into_inner();
    let fridge_with_compartments = interactor
        .handle_get_fridge(&user.user_id, &fridge_id)
        .await?;
    let response = get_schema::GetFridgeResponse::from(fridge_with_compartments);
    Ok(HttpResponse::Ok().json(response))
}

pub async fn delete_fridge<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<HttpResponse> {
    let fridge_id = path.into_inner();
    interactor
        .handle_delete_fridge(&user.user_id, &fridge_id)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}

pub async fn list_fridges<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    user: AuthenticatedUser,
) -> Result<HttpResponse> {
    let response = interactor.handle_list_fridges(&user.user_id).await?;
    Ok(HttpResponse::Ok().json(list_schema::ListFridgesResponse::from(response)))
}
