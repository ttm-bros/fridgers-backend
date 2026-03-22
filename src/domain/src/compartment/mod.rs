mod compartment_id;
mod compartment_name;

use std::marker::PhantomData;

pub use compartment_id::CompartmentId;
pub use compartment_name::CompartmentName;

use crate::fridge::FridgeId;

#[derive(Debug, Clone, PartialEq)]
pub struct Compartment {
    pub id: CompartmentId,
    pub fridge_id: FridgeId,
    pub name: CompartmentName,
    _hide_default_constructor: PhantomData<()>,
}

impl Compartment {
    pub fn new(id: CompartmentId, fridge_id: FridgeId, name: CompartmentName) -> Self {
        Self {
            id,
            fridge_id,
            name,
            _hide_default_constructor: PhantomData,
        }
    }
}
