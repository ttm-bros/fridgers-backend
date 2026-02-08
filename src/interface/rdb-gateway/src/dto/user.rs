use fridgers_backend_domain::user::{User, UserId, UserName};
use fridgers_backend_use_case::Result;
use sqlx::FromRow;
use uuid::Uuid;

/// usersテーブルの行をマッピングするDTO
#[derive(FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
}

impl UserRow {
    /// ドメインモデルのUserに変換する
    pub fn into_domain(self) -> Result<User> {
        let user_id = UserId::from(self.id);
        let user_name = UserName::try_from(self.name)?;
        Ok(User::new(user_id, user_name))
    }
}
