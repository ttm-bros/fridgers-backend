#[derive(Debug, Clone, PartialEq)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("User name cannot be empty".to_string());
        }
        Ok(Self(name))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
