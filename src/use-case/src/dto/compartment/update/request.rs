use fridgers_backend_domain::compartment::{CompartmentId, CompartmentName};
use fridgers_backend_domain::fridge::FridgeId;
use std::marker::PhantomData;

pub struct UpdateCompartmentRequest {
    pub compartment_id: CompartmentId,
    pub fridge_id: FridgeId,
    pub name: CompartmentName,
    _hide_default_constructor: PhantomData<()>,
}

impl UpdateCompartmentRequest {
    pub fn new(compartment_id: CompartmentId, fridge_id: FridgeId, name: CompartmentName) -> Self {
        Self {
            compartment_id,
            fridge_id,
            name,
            _hide_default_constructor: PhantomData,
        }
    }
}
