use crate::error::{Error, Result};
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct FridgeName {
    value: String,
    _hide_default_constructor: PhantomData<()>,
}

impl FridgeName {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TryFrom<String> for FridgeName {
    type Error = Error;

    fn try_from(name: String) -> Result<Self> {
        if name.is_empty() {
            return Err(Error::InvalidLengthRange(
                "Fridge name cannot be empty".to_string(),
            ));
        }
        if name.len() > 50 {
            return Err(Error::InvalidLengthRange(
                "Fridge name cannot exceed 50 characters".to_string(),
            ));
        }
        Ok(Self {
            value: name,
            _hide_default_constructor: PhantomData,
        })
    }
}
