use crate::error::Result;
use crate::schema::user::register::{self as schema};
use actix_web::{HttpResponse, web};
use fridgers_backend_domain::user::{Email, UserId, UserName};
use fridgers_backend_use_case::{self as use_case, Interactor, Repository, dto::user::register as dto};
use std::sync::Arc;

pub async fn register_user<R: Repository + 'static>(
    interactor: web::Data<Arc<Interactor<R>>>,
    req: web::Json<schema::RegisterUserRequest>,
) -> Result<HttpResponse> {
    // ドメインオブジェクトの生成
    let user_id = UserId::new();
    let user_name = UserName::try_from(req.name.clone()).map_err(use_case::Error::from)?;
    let email = Email::try_from(req.email.clone()).map_err(use_case::Error::from)?;

    // use-case DTOを作成
    let use_case_request = dto::RegisterUserRequest::new(user_id, user_name, email, req.password.clone());

    // interactorを通じてユースケースを実行
    let user = interactor.handle_register_user(use_case_request).await?;

    let response = schema::RegisterUserResponse::from(user);
    Ok(HttpResponse::Created().json(response))
}
