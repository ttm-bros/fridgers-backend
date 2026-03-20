use crate::error::Result;
use crate::schema::auth::login::{self as schema};
use actix_web::{HttpResponse, web};
use fridgers_backend_use_case::{Interactor, Repository, dto::auth::login as dto};
use std::sync::Arc;

pub async fn login<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    req: web::Json<schema::LoginRequest>,
) -> Result<HttpResponse> {
    let use_case_request = dto::LoginRequest::new(req.email.clone(), req.password.clone());

    let token = interactor.handle_login(use_case_request).await?;

    let response = schema::LoginResponse::new(token);
    Ok(HttpResponse::Ok().json(response))
}
