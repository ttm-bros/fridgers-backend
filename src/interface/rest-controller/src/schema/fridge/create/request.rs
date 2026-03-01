use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateFridgeRequest {
    pub name: String,
    pub owner_user_id: String,
}
