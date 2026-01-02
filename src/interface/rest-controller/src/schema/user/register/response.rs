use fridgers_backend_domain::user::User;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub id: String,
    pub name: String,
}

impl From<User> for RegisterUserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.value().to_string(),
            name: user.name.value().to_string(),
        }
    }
}
