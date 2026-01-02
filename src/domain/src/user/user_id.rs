use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct UserId {
    value: String,
    _hide_default_constructor: PhantomData<()>,
}

impl UserId {
    pub fn new(id: String) -> Result<Self, String> {
        if id.is_empty() {
            return Err("User ID cannot be empty".to_string());
        }
        Ok(Self {
            value: id,
            _hide_default_constructor: PhantomData,
        })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
