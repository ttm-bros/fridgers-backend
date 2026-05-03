use fridgers_backend_use_case::dto::fridge::list::ListFridgesResponse as UseCaseResponse;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ListFridgesResponse {
    pub fridges: Vec<FridgeListItem>,
}

#[derive(Debug, Serialize)]
pub struct FridgeListItem {
    pub id: String,
    pub name: String,
    pub owner_user_id: String,
}

impl From<UseCaseResponse> for ListFridgesResponse {
    fn from(response: UseCaseResponse) -> Self {
        Self {
            fridges: response
                .fridges
                .into_iter()
                .map(|f| FridgeListItem {
                    id: f.id.value().to_string(),
                    name: f.name.value().to_string(),
                    owner_user_id: f.owner_user_id.value().to_string(),
                })
                .collect(),
        }
    }
}
