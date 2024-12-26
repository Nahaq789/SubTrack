use std::{fmt::Display, str::FromStr};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Password {
    value: String,
}

#[derive(Debug, Error, PartialEq)]
pub enum PasswordError {
    #[error("Validate Error")]
    ValidateFailed,
}

impl Password {
    fn validate_password(password: &str) -> bool {
        if password.len() < 8 {
            return false;
        }

        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());

        match (has_lowercase || has_uppercase) && has_digit {
            true => true,
            false => false,
        }
    }
}

impl FromStr for Password {
    type Err = PasswordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::validate_password(s) {
            true => {
                let password = { Self { value: s.into() } };
                Ok(password)
            }
            false => Err(PasswordError::ValidateFailed),
        }
    }
}

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_success() {
        let test_cases = vec![
            "Password123",
            "Mix3dPassw0rd",
            "MixedCase123",
            "12cdefghij",
            "abcd1234",
            "ABCD1234",
            "abc123DEF",
            "123456aB",
        ];
        for value in test_cases {
            let result = Password::validate_password(value);
            assert!(result)
        }
    }

    #[test]
    fn test_validate_password_failed() {
        let test_cases = vec![
            "",
            "a",
            "1",
            "abcdef1",
            "   ",
            "abc def",
            "!@#$%^&",
            "12345678",
            "abcdefghijk",
            "パスワード123",
            "pass\nword",
            "pass\tword",
        ];
        for value in test_cases {
            let result = Password::validate_password(value);
            assert!(!result)
        }
    }

    #[test]
    fn test_from_str_success() {
        let password = "Abcdefg123";
        let result = Password::from_str(password);
        assert!(result.is_ok());
        assert_eq!(password.to_owned(), result.unwrap().value)
    }

    #[test]
    fn test_from_str_failed() {
        let error_message = "Validate Error";
        let result = Password::from_str("hoge");
        assert!(result.is_err());
        assert_eq!(error_message.to_owned(), result.unwrap_err().to_string())
    }
}
