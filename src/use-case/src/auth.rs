use crate::{Error, Result};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// JWT設定
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub expiry_hours: u64,
}

/// JWTクレーム
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject（ユーザーID）
    pub sub: String,
    /// 有効期限（UNIXタイムスタンプ）
    pub exp: usize,
    /// 発行日時（UNIXタイムスタンプ）
    pub iat: usize,
}

/// JWTアクセストークンを生成する
pub fn encode_token(user_id: &str, config: &JwtConfig) -> Result<String> {
    let now = Utc::now();
    let exp = now + chrono::Duration::hours(config.expiry_hours as i64);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| Error::ExternalServer(format!("Failed to encode JWT: {}", e)))
}

/// JWTアクセストークンを検証・デコードする
pub fn decode_token(token: &str, config: &JwtConfig) -> Result<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| Error::Unauthorized(format!("Invalid token: {}", e)))
}
