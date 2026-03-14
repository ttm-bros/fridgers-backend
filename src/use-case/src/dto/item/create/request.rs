use chrono::NaiveDate;
use fridgers_backend_domain::compartment::CompartmentId;
use fridgers_backend_domain::item::{ItemId, ItemName, ItemUnit};
use std::marker::PhantomData;

pub struct CreateItemRequest {
    pub item_id: ItemId,
    pub compartment_id: CompartmentId,
    pub name: ItemName,
    pub quantity: f64,
    pub unit: ItemUnit,
    pub expires_at: Option<NaiveDate>,
    _hide_default_constructor: PhantomData<()>,
}

impl CreateItemRequest {
    pub fn new(
        item_id: ItemId,
        compartment_id: CompartmentId,
        name: ItemName,
        quantity: f64,
        unit: ItemUnit,
        expires_at: Option<NaiveDate>,
    ) -> Self {
        Self {
            item_id,
            compartment_id,
            name,
            quantity,
            unit,
            expires_at,
            _hide_default_constructor: PhantomData,
        }
    }
}
