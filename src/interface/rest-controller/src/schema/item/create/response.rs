use chrono::{DateTime, NaiveDate, Utc};
use fridgers_backend_domain::item::Item;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateItemResponse {
    pub id: String,
    pub compartment_id: String,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub expires_at: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Item> for CreateItemResponse {
    fn from(item: Item) -> Self {
        Self {
            id: item.id.value().to_string(),
            compartment_id: item.compartment_id.value().to_string(),
            name: item.name.value().to_string(),
            quantity: item.quantity,
            unit: item.unit.value().to_string(),
            expires_at: item.expires_at,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
