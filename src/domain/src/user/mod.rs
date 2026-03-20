mod email;
mod password_hash;
mod raw_password;
mod user_id;
mod user_name;

pub use email::Email;
pub use password_hash::PasswordHash;
pub use raw_password::RawPassword;
pub use user_id::UserId;
pub use user_name::UserName;

use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub email: Email,
    pub password_hash: PasswordHash,
    _hide_default_constructor: PhantomData<()>,
}

impl User {
    pub fn new(id: UserId, name: UserName, email: Email, password_hash: PasswordHash) -> Self {
        Self {
            id,
            name,
            email,
            password_hash,
            _hide_default_constructor: PhantomData,
        }
    }
}
