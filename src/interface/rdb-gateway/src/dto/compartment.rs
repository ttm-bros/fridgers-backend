use fridgers_backend_domain::compartment::{Compartment, CompartmentId, CompartmentName};
use fridgers_backend_domain::fridge::FridgeId;
use fridgers_backend_use_case::Error;
use fridgers_backend_use_case::Result;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct CompartmentRow {
    pub id: Uuid,
    pub fridge_id: Uuid,
    pub name: String,
}

impl TryFrom<CompartmentRow> for Compartment {
    type Error = Error;

    fn try_from(row: CompartmentRow) -> Result<Self> {
        let id = CompartmentId::from(row.id);
        let fridge_id = FridgeId::from(row.fridge_id);
        let name =
            CompartmentName::try_from(row.name).map_err(fridgers_backend_use_case::Error::from)?;
        Ok(Compartment::new(id, fridge_id, name))
    }
}
