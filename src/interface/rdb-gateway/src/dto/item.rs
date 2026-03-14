use chrono::{DateTime, NaiveDate, Utc};
use fridgers_backend_domain::compartment::CompartmentId;
use fridgers_backend_domain::item::{Item, ItemId, ItemName, ItemUnit};
use fridgers_backend_use_case::Error;
use fridgers_backend_use_case::Result;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct ItemRow {
    pub id: Uuid,
    pub compartment_id: Uuid,
    pub name: String,
    pub quantity: f64,
    pub unit: String,
    pub expires_at: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<ItemRow> for Item {
    type Error = Error;

    fn try_from(row: ItemRow) -> Result<Self> {
        let id = ItemId::from(row.id);
        let compartment_id = CompartmentId::from(row.compartment_id);
        let name = ItemName::try_from(row.name).map_err(fridgers_backend_use_case::Error::from)?;
        let unit = ItemUnit::try_from(row.unit).map_err(fridgers_backend_use_case::Error::from)?;
        Ok(Item::new(
            id,
            compartment_id,
            name,
            row.quantity,
            unit,
            row.expires_at,
            row.created_at,
            row.updated_at,
        ))
    }
}
