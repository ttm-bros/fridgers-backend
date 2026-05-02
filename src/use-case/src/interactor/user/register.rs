use crate::dto::user::register::RegisterUserRequest;
use crate::interactor::Interactor;
use crate::repository::Repository;
use crate::{Error, Result};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use fridgers_backend_domain::user::{PasswordHash, User};

impl<R: Repository> Interactor<R> {
    /// ユーザーを登録する
    pub async fn handle_register_user(&self, request: RegisterUserRequest) -> Result<User> {
        // メールアドレスの重複チェック
        let existing = self
            .repository
            .find_user_by_email(request.email.value())
            .await?;
        if existing.is_some() {
            return Err(Error::AlreadyExist("Email already registered".to_string()));
        }

        // パスワードをハッシュ化
        let salt = SaltString::generate(&mut OsRng);
        let hashed = Argon2::default()
            .hash_password(request.password.as_bytes(), &salt)
            .map_err(|e| Error::ExternalServer(format!("Failed to hash password: {}", e)))?;
        let password_hash = PasswordHash::from(hashed.to_string());

        // ユーザーエンティティを作成
        let user = User::new(
            request.user_id,
            request.user_name,
            request.email,
            password_hash,
        );

        // リポジトリに保存
        self.repository.save_user(&user).await?;

        Ok(user)
    }
}
