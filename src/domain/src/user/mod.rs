mod raw_password;
mod user_id;
mod user_name;

pub use raw_password::RawPassword;
pub use user_id::UserId;
pub use user_name::UserName;

use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    _hide_default_constructor: PhantomData<()>,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self {
            id,
            name,
            _hide_default_constructor: PhantomData,
        }
    }
}
