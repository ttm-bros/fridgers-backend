use fridgers_backend_domain::user::{User, UserId, UserName};
use fridgers_backend_use_case::Error;
use sqlx::FromRow;
use uuid::Uuid;

/// usersテーブルの行をマッピングするDTO
#[derive(FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
}

impl TryFrom<UserRow> for User {
    type Error = Error;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let user_id = UserId::from(row.id);
        let user_name = UserName::try_from(row.name)?;
        Ok(User::new(user_id, user_name))
    }
}
