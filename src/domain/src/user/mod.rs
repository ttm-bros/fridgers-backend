mod user_id;
mod user_name;

pub use user_id::UserId;
pub use user_name::UserName;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }
}
