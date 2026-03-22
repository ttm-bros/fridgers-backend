use chrono::NaiveDate;
use fridgers_backend_domain::item::{ItemId, ItemName, ItemUnit};
use std::marker::PhantomData;

pub struct UpdateItemRequest {
    pub item_id: ItemId,
    pub name: ItemName,
    pub quantity: f64,
    pub unit: ItemUnit,
    pub expires_at: Option<NaiveDate>,
    _hide_default_constructor: PhantomData<()>,
}

impl UpdateItemRequest {
    pub fn new(
        item_id: ItemId,
        name: ItemName,
        quantity: f64,
        unit: ItemUnit,
        expires_at: Option<NaiveDate>,
    ) -> Self {
        Self {
            item_id,
            name,
            quantity,
            unit,
            expires_at,
            _hide_default_constructor: PhantomData,
        }
    }
}
