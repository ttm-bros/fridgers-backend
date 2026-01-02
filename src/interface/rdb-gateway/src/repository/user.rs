use fridgers_backend_domain::user::User;
use fridgers_backend_use_case::repository::UserRepository;
use fridgers_backend_use_case::Result;

/// インメモリユーザーリポジトリ実装
/// 将来的にはデータベース実装に置き換える
pub struct InMemoryUserRepository;

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self
    }
}

impl UserRepository for InMemoryUserRepository {
    fn save(&self, _user: &User) -> Result<()> {
        // 仮実装: メモリ内保存（実際はDBに保存）
        // TODO: データベース実装
        Ok(())
    }

    fn find_by_id(&self, _id: &str) -> Result<Option<User>> {
        // 仮実装: メモリから検索（実際はDBから検索）
        // TODO: データベース実装
        Ok(None)
    }
}
