use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ItemId {
    value: Uuid,
    _hide_default_constructor: PhantomData<()>,
}

impl ItemId {
    pub fn new() -> Self {
        Self::from(Uuid::new_v4())
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl From<Uuid> for ItemId {
    fn from(id: Uuid) -> Self {
        Self {
            value: id,
            _hide_default_constructor: PhantomData,
        }
    }
}
