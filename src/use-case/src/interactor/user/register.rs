use crate::dto::user::register::RegisterUserRequest;
use crate::interactor::Interactor;
use crate::repository::Repository;
use crate::Result;
use fridgers_backend_domain::user::User;

impl<R: Repository> Interactor<R> {
    /// ユーザーを登録する
    pub async fn handle_register_user(&self, request: RegisterUserRequest) -> Result<User> {
        // ユーザーエンティティを作成
        let user = User::new(request.user_id, request.user_name);

        // リポジトリに保存
        self.repository.save_user(&user).await?;

        Ok(user)
    }
}
