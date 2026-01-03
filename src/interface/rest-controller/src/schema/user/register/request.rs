use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUserRequest {
    pub name: String,
}
