use crate::extractor::extract_bearer_token;
use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use fridgers_backend_use_case::auth::{JwtConfig, decode_token};
use std::future::{Ready, ready};

/// 認証済ユーザー。Authorization ヘッダの JWT を復号して取得する。
///
/// ハンドラの引数として宣言すれば Actix が自動で抽出する。
/// 抽出に失敗した場合は 401 を返す。
pub struct AuthenticatedUser {
    pub user_id: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = crate::error::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(authenticate(req))
    }
}

fn authenticate(req: &HttpRequest) -> Result<AuthenticatedUser, crate::error::Error> {
    let token = extract_bearer_token(req)?;
    let jwt_config = req
        .app_data::<web::Data<JwtConfig>>()
        .expect("JwtConfig must be registered in app_data");
    let claims = decode_token(token, jwt_config.get_ref())?;
    Ok(AuthenticatedUser {
        user_id: claims.sub,
    })
}
