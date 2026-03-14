use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCompartmentRequest {
    pub name: String,
}
