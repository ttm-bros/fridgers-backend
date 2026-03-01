use fridgers_backend_domain::fridge::Fridge;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateFridgeResponse {
    pub id: String,
    pub name: String,
    pub owner_user_id: String,
}

impl From<Fridge> for CreateFridgeResponse {
    fn from(fridge: Fridge) -> Self {
        Self {
            id: fridge.id.value().to_string(),
            name: fridge.name.value().to_string(),
            owner_user_id: fridge.owner_user_id.value().to_string(),
        }
    }
}
