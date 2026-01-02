use crate::error::{Error, Result};
use std::marker::PhantomData;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct UserId {
    value: Uuid,
    _hide_default_constructor: PhantomData<()>,
}

impl UserId {
    pub fn new(id: Uuid) -> Self {
        Self {
            value: id,
            _hide_default_constructor: PhantomData,
        }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl TryFrom<String> for UserId {
    type Error = Error;

    fn try_from(id: String) -> Result<Self> {
        let uuid = Uuid::parse_str(&id)
            .map_err(|e| Error::InvalidFormat(format!("Invalid UUID format: {}", e)))?;
        Ok(Self::new(uuid))
    }
}
