use crate::auth;
use crate::dto::auth::login::LoginRequest;
use crate::interactor::Interactor;
use crate::repository::Repository;
use crate::{Error, Result};
use argon2::{Argon2, PasswordHash, PasswordVerifier};

impl<R: Repository> Interactor<R> {
    /// ログイン処理: メールアドレスでユーザーを検索し、パスワードを検証してJWTを発行する
    pub async fn handle_login(&self, request: LoginRequest) -> Result<String> {
        // メールアドレスでユーザーを検索
        let user = self
            .repository
            .find_user_by_email(&request.email)
            .await?
            .ok_or_else(|| Error::Unauthorized("Invalid email or password".to_string()))?;

        // パスワードを検証
        let parsed_hash = PasswordHash::new(user.password_hash.value())
            .map_err(|e| Error::ExternalServer(format!("Failed to parse password hash: {}", e)))?;

        Argon2::default()
            .verify_password(request.password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::Unauthorized("Invalid email or password".to_string()))?;

        // JWTトークンを生成
        let token = auth::encode_token(&user.id.value().to_string(), &self.jwt_config)?;

        Ok(token)
    }
}
