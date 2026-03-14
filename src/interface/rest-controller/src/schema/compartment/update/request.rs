use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateCompartmentRequest {
    pub name: String,
}
