use crate::error::{Error, Result};
use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq)]
pub struct Email {
    value: String,
    _hide_default_constructor: PhantomData<()>,
}

impl Email {
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl TryFrom<String> for Email {
    type Error = Error;

    fn try_from(email: String) -> Result<Self> {
        if email.is_empty() {
            return Err(Error::InvalidFormat("Email cannot be empty".to_string()));
        }

        // 基本的なフォーマットバリデーション: @ が1つ含まれ、前後が空でないこと
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
            return Err(Error::InvalidFormat("Invalid email format".to_string()));
        }

        // ドメイン部分に . が含まれること
        if !parts[1].contains('.') {
            return Err(Error::InvalidFormat("Invalid email format".to_string()));
        }

        Ok(Self {
            value: email,
            _hide_default_constructor: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::try_from("user@example.com".to_string());
        assert!(email.is_ok());
        assert_eq!(email.unwrap().value(), "user@example.com");
    }

    #[test]
    fn test_empty_email() {
        let email = Email::try_from("".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_no_at_sign() {
        let email = Email::try_from("userexample.com".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_no_domain_dot() {
        let email = Email::try_from("user@example".to_string());
        assert!(email.is_err());
    }

    #[test]
    fn test_empty_local_part() {
        let email = Email::try_from("@example.com".to_string());
        assert!(email.is_err());
    }
}
