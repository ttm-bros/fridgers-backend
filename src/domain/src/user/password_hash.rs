use std::marker::PhantomData;

/// ハッシュ済みパスワードのラッパー
#[derive(Debug, Clone, PartialEq)]
pub struct PasswordHash {
    value: String,
    _hide_default_constructor: PhantomData<()>,
}

impl PasswordHash {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl From<String> for PasswordHash {
    fn from(hash: String) -> Self {
        Self {
            value: hash,
            _hide_default_constructor: PhantomData,
        }
    }
}
