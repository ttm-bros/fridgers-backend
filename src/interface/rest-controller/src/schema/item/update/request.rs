use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub expires_at: Option<NaiveDate>,
}
