use fridgers_backend_domain::compartment::Compartment;
use fridgers_backend_domain::fridge::Fridge;
use fridgers_backend_domain::item::Item;

pub struct FridgeWithCompartments {
    pub fridge: Fridge,
    pub compartments: Vec<CompartmentWithItems>,
}

pub struct CompartmentWithItems {
    pub compartment: Compartment,
    pub items: Vec<Item>,
}
