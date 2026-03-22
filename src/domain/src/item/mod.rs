mod item_id;

use std::marker::PhantomData;

pub use item_id::ItemId;

use crate::compartment::CompartmentId;
use crate::string::define_string;
use chrono::{DateTime, NaiveDate, Utc};

define_string!(ItemName, max = 100, validator = |c: char| !c.is_control());
define_string!(ItemUnit, max = 20, validator = |c: char| !c.is_control());

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub id: ItemId,
    pub compartment_id: CompartmentId,
    pub name: ItemName,
    pub quantity: f64,
    pub unit: ItemUnit,
    pub expires_at: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    _hide_default_constructor: PhantomData<()>,
}

impl Item {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ItemId,
        compartment_id: CompartmentId,
        name: ItemName,
        quantity: f64,
        unit: ItemUnit,
        expires_at: Option<NaiveDate>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            compartment_id,
            name,
            quantity,
            unit,
            expires_at,
            created_at,
            updated_at,
            _hide_default_constructor: PhantomData,
        }
    }
}
