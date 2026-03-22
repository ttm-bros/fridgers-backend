use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct CompartmentId {
    value: Uuid,
    _hide_default_constructor: PhantomData<()>,
}

impl CompartmentId {
    pub fn new() -> Self {
        Self::from(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl From<Uuid> for CompartmentId {
    fn from(id: Uuid) -> Self {
        Self {
            value: id,
            _hide_default_constructor: PhantomData,
        }
    }
}
