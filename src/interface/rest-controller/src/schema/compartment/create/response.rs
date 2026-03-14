use fridgers_backend_domain::compartment::Compartment;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CreateCompartmentResponse {
    pub id: String,
    pub fridge_id: String,
    pub name: String,
}

impl From<Compartment> for CreateCompartmentResponse {
    fn from(compartment: Compartment) -> Self {
        Self {
            id: compartment.id.value().to_string(),
            fridge_id: compartment.fridge_id.value().to_string(),
            name: compartment.name.value().to_string(),
        }
    }
}
