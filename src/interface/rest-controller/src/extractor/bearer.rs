use actix_web::HttpRequest;
use fridgers_backend_use_case::{Error, Result};

/// Authorization ヘッダから Bearer トークンを抽出する
///
/// scheme は RFC 7235 に従い大文字小文字を区別しない。
pub fn extract_bearer_token(req: &HttpRequest) -> Result<&str> {
    let header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Unauthorized("Missing Authorization header".into()))?;

    let (scheme, token) = header
        .split_once(' ')
        .ok_or_else(|| Error::Unauthorized("Invalid Authorization header".into()))?;

    if !scheme.eq_ignore_ascii_case("bearer") {
        return Err(Error::Unauthorized(
            "Authorization scheme must be Bearer".into(),
        ));
    }

    if token.is_empty() {
        return Err(Error::Unauthorized("Empty Bearer token".into()));
    }

    Ok(token)
}
