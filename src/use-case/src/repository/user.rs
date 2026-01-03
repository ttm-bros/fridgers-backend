use crate::Result;
use fridgers_backend_domain::user::User;

/// ユーザーリポジトリのインターフェイス
pub trait UserRepository: Send + Sync {
    /// ユーザーを保存する
    fn save(&self, user: &User) -> Result<()>;

    /// ユーザーIDでユーザーを検索する
    fn find_by_id(&self, id: &str) -> Result<Option<User>>;
}
