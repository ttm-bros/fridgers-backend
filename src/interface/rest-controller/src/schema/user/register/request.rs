use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub id: String,
    pub name: String,
}
