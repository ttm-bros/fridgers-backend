#[derive(Debug, Clone, PartialEq)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Result<Self, String> {
        if id.is_empty() {
            return Err("User ID cannot be empty".to_string());
        }
        Ok(Self(id))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
