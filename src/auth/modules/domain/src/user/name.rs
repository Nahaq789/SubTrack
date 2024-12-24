use std::{fmt::Display, str::FromStr};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name {
    value: String,
}

#[derive(Debug, Error)]
pub enum NameError {
    #[error("Invalid validate name {0}")]
    InvalidValidateName(String),
}

impl Name {
    fn validate_name(name: &str) -> bool {
        if name.len() > 20 {
            return false;
        }
        true
    }
}

impl FromStr for Name {
    type Err = NameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::validate_name(s) {
            true => Ok(Self {
                value: s.to_string(),
            }),
            false => Err(NameError::InvalidValidateName(s.to_string())),
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_success() {
        let result = Name::from_str("hoge");
        assert!(result.is_ok());
        assert_eq!(String::from("hoge"), result.unwrap().value)
    }

    #[test]
    fn test_display_success() {
        let result = Name::from_str("hoge").unwrap();
        assert_eq!("hoge".to_string(), result.to_string())
    }
}
