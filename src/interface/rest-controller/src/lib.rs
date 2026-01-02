pub mod schema;

use actix_web::{web, HttpResponse, Responder};
use fridgers_backend_domain::user::{User, UserId, UserName};
use schema::user::register::{RegisterUserRequest, RegisterUserResponse};

pub async fn register_user(req: web::Json<RegisterUserRequest>) -> impl Responder {
    // ドメインオブジェクトの生成
    let user_id = match UserId::try_from(req.id.clone()) {
        Ok(id) => id,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }));
        }
    };

    let user_name = match UserName::new(req.name.clone()) {
        Ok(name) => name,
        Err(e) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": e.to_string()
            }));
        }
    };

    let user = User::new(user_id, user_name);

    // ここで将来的にはユースケースを呼び出す
    // 現在はモックとして直接レスポンスを返す
    let response = RegisterUserResponse::from(user);
    HttpResponse::Created().json(response)
}

// コントローラーの設定を公開する関数
pub fn configure_users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(register_user))
    );
}
