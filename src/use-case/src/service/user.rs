use crate::repository::UserRepository;
use crate::Result;
use fridgers_backend_domain::user::{User, UserId, UserName};

/// ユーザー登録ユースケース
pub struct RegisterUserUseCase<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> RegisterUserUseCase<R> {
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    /// ユーザーを登録する
    pub fn execute(&self, user_id: UserId, user_name: UserName) -> Result<User> {
        // ユーザーエンティティを作成
        let user = User::new(user_id, user_name);

        // ここで将来的にビジネスルールを追加
        // 例: 既存ユーザーのチェック、バリデーションなど

        // リポジトリに保存
        self.user_repository.save(&user)?;

        Ok(user)
    }
}
