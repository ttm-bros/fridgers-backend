use crate::Result;
use fridgers_backend_domain::user::User;

/// リポジトリのインターフェイス
pub trait Repository: Send + Sync {
    /// ユーザーを保存する
    fn save_user(&self, user: &User) -> impl Future<Output = Result<()>> + Send;

    /// ユーザーIDでユーザーを検索する
    fn find_user_by_id(&self, id: &str) -> impl Future<Output = Result<Option<User>>> + Send;

    /// ユーザーを削除する
    fn delete_user(&self, id: &str) -> impl Future<Output = Result<()>> + Send;
}
