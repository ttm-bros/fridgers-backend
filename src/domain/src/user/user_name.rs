use crate::error::{Error, Result};
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct UserName {
    value: String,
    _hide_default_constructor: PhantomData<()>,
}

impl UserName {
    pub fn new(name: String) -> Result<Self> {
        if name.is_empty() {
            return Err(Error::InvalidLengthRange(
                "User name cannot be empty".to_string(),
            ));
        }
        Ok(Self {
            value: name,
            _hide_default_constructor: PhantomData,
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
