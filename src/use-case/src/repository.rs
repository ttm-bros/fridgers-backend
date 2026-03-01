use crate::Result;
use fridgers_backend_domain::fridge::Fridge;
use fridgers_backend_domain::user::User;

/// リポジトリのインターフェイス
pub trait Repository: Send + Sync {
    /// ユーザーを保存する
    fn save_user(&self, user: &User) -> impl Future<Output = Result<()>> + Send;

    /// ユーザーIDでユーザーを検索する
    fn find_user_by_id(&self, id: &str) -> impl Future<Output = Result<Option<User>>> + Send;

    /// ユーザーを削除する
    fn delete_user(&self, id: &str) -> impl Future<Output = Result<()>> + Send;

    /// メールアドレスでユーザーを検索する
    fn find_user_by_email(&self, email: &str) -> impl Future<Output = Result<Option<User>>> + Send;

    /// 冷蔵庫を保存する
    fn save_fridge(&self, fridge: &Fridge) -> impl Future<Output = Result<()>> + Send;

    /// 冷蔵庫IDで冷蔵庫を検索する
    fn find_fridge_by_id(&self, id: &str) -> impl Future<Output = Result<Option<Fridge>>> + Send;

    /// 冷蔵庫を削除する
    fn delete_fridge(&self, id: &str) -> impl Future<Output = Result<()>> + Send;
}
