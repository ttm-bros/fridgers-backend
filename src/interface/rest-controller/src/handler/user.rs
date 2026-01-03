use crate::error::Result;
use crate::schema::user::register::{RegisterUserRequest, RegisterUserResponse};
use actix_web::{HttpResponse, post, web};
use fridgers_backend_domain::user::{UserId, UserName};
use fridgers_backend_use_case::{self as use_case, Interactor};
use std::sync::Arc;

#[post("/v1/users")]
pub async fn register_user(
    interactor: web::Data<Arc<Interactor>>,
    req: web::Json<RegisterUserRequest>,
) -> Result<HttpResponse> {
    // ドメインオブジェクトの生成
    let user_id = UserId::try_from(req.id.clone()).map_err(use_case::Error::from)?;

    let user_name = UserName::try_new(req.name.clone()).map_err(use_case::Error::from)?;

    // interactorを通じてユースケースを実行
    let user = interactor.handle_register_user(user_id, user_name)?;

    let response = RegisterUserResponse::from(user);
    Ok(HttpResponse::Created().json(response))
}
