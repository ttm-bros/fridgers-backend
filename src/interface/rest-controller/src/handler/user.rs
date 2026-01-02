use actix_web::{post, web, HttpResponse, Responder};
use fridgers_backend_domain::user::{UserId, UserName};
use fridgers_backend_use_case::interactor::FridgersRestInteractor;
use crate::schema::user::register::{RegisterUserRequest, RegisterUserResponse};
use std::sync::Arc;

#[post("/v1/users")]
pub async fn register_user(
    interactor: web::Data<Arc<FridgersRestInteractor>>,
    req: web::Json<RegisterUserRequest>,
) -> impl Responder {
    // ドメインオブジェクトの生成
    let user_id = match UserId::try_from(req.id.clone()) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }));
        }
    };

    let user_name = match UserName::try_new(req.name.clone()) {
        Ok(name) => name,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }));
        }
    };

    // interactorを通じてユースケースを実行
    let user = match interactor.handle_register_user(user_id, user_name) {
        Ok(user) => user,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }));
        }
    };

    let response = RegisterUserResponse::from(user);
    HttpResponse::Created().json(response)
}
