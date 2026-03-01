use fridgers_backend_domain::user::{Email, PasswordHash, User, UserId, UserName};
use fridgers_backend_use_case::Error;
use sqlx::FromRow;
use uuid::Uuid;

/// usersテーブルの行をマッピングするDTO
#[derive(FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl TryFrom<UserRow> for User {
    type Error = Error;

    fn try_from(row: UserRow) -> Result<Self, Self::Error> {
        let user_id = UserId::from(row.id);
        let user_name = UserName::try_from(row.name)?;
        let email = Email::try_from(row.email)?;
        let password_hash = PasswordHash::from(row.password_hash);
        Ok(User::new(user_id, user_name, email, password_hash))
    }
}
