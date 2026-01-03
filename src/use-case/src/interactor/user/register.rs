use crate::dto::user::register::RegisterUserRequest;
use crate::interactor::Interactor;
use crate::Result;
use fridgers_backend_domain::user::User;

impl Interactor {
    /// ユーザーを登録する
    pub fn handle_register_user(&self, request: RegisterUserRequest) -> Result<User> {
        // ユーザーエンティティを作成
        let user = User::new(request.user_id, request.user_name);

        // ここで将来的にビジネスルールを追加
        // 例: 既存ユーザーのチェック、バリデーションなど

        // リポジトリに保存
        self.repository.save(&user)?;

        Ok(user)
    }
}
