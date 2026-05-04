use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateFridgeRequest {
    pub name: String,
}
