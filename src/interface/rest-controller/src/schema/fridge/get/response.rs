use chrono::{DateTime, NaiveDate, Utc};
use fridgers_backend_use_case::dto::fridge::get::FridgeWithCompartments;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetFridgeResponse {
    pub id: String,
    pub name: String,
    pub owner_user_id: String,
    pub compartments: Vec<CompartmentResponse>,
}

#[derive(Debug, Serialize)]
pub struct CompartmentResponse {
    pub id: String,
    pub name: String,
    pub items: Vec<ItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub id: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub expires_at: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<FridgeWithCompartments> for GetFridgeResponse {
    fn from(data: FridgeWithCompartments) -> Self {
        Self {
            id: data.fridge.id.value().to_string(),
            name: data.fridge.name.value().to_string(),
            owner_user_id: data.fridge.owner_user_id.value().to_string(),
            compartments: data
                .compartments
                .into_iter()
                .map(|c| CompartmentResponse {
                    id: c.compartment.id.value().to_string(),
                    name: c.compartment.name.value().to_string(),
                    items: c
                        .items
                        .into_iter()
                        .map(|i| ItemResponse {
                            id: i.id.value().to_string(),
                            name: i.name.value().to_string(),
                            quantity: i.quantity,
                            unit: i.unit.value().to_string(),
                            expires_at: i.expires_at,
                            created_at: i.created_at,
                            updated_at: i.updated_at,
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
