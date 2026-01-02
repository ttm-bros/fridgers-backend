use crate::Result;
use crate::interactor::FridgersRestInteractor;
use crate::repository::UserRepository;
use fridgers_backend_domain::user::{User, UserId, UserName};

impl FridgersRestInteractor {
    /// ユーザーを登録する
    pub fn handle_register_user(&self, user_id: UserId, user_name: UserName) -> Result<User> {
        // ユーザーエンティティを作成
        let user = User::new(user_id, user_name);

        // ここで将来的にビジネスルールを追加
        // 例: 既存ユーザーのチェック、バリデーションなど

        // リポジトリに保存
        self.repository.save(&user)?;

        Ok(user)
    }
}
