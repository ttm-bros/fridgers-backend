use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct FridgeId {
    value: Uuid,
    _hide_default_constructor: PhantomData<()>,
}

impl FridgeId {
    pub fn new() -> Self {
        Self::from(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl From<Uuid> for FridgeId {
    fn from(id: Uuid) -> Self {
        Self {
            value: id,
            _hide_default_constructor: PhantomData,
        }
    }
}
