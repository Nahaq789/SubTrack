use std::{fmt::Display, str::FromStr};

use regex::Regex;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email {
    value: String,
}

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Regex compile failed {0}")]
    RegexCompilationFailed(String),

    #[error("Validate Error")]
    ValidateFailed,
}

impl Email {
    fn validate_email(email: &str) -> Result<bool, EmailError> {
        let regex = Regex::new(r"^[a-z0-9]([a-z0-9._%+-]{0,61}[a-z0-9])?@[a-z0-9-]{1,63}(\.[a-z0-9-]{1,63})*\.[a-z]{2,6}$")
            .map_err(|e| EmailError::RegexCompilationFailed(e.to_string()))?;
        Ok(regex.is_match(email))
    }
}

impl FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::validate_email(s)? {
            true => Ok(Self {
                value: s.to_string(),
            }),
            false => Err(EmailError::ValidateFailed),
        }
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_success() {
        let email = "hoge@email.com";
        let result = Email::from_str(email);
        assert!(result.is_ok());
        assert_eq!(email, &result.unwrap().value)
    }

    #[test]
    fn test_email_validate_success() {
        let test_cases = vec![
            "simple@example.com",
            "very.common@example.com",
            "disposable.style.email.with+symbol@example.com",
            "other.email-with-hyphen@example.com",
            "fully-qualified-domain@example.com",
            "user.name+tag+sorting@example.com",
            "x@example.com",
            "example-indeed@strange-example.com",
            "user%example.com@example.org",
        ];

        for value in test_cases {
            let result = Email::validate_email(value);
            assert!(result.is_ok());

            let result = result.unwrap();
            assert!(result)
        }
    }

    #[test]
    fn test_email_validate_failed() {
        let test_cases = vec![
            "Password123",
            "StrongP@ssw0rd",
            "aA1!bB2@cC3#",
            "7CharAZ",
            "abcdefG",
            "ABCDEFg",
            "Mix3dPassw0rd",
            "LongPasswordWithUpperAndLowerCase",
            "ShortPW1",
            "UPPER123lower",
            "lower123UPPER",
            "PassWord@2023",
            "Aa1!Bb2@Cc3#Dd4$",
            "ThIs1sAV3ryL0ngAndC0mpl3xP@ssw0rd",
        ];

        for value in test_cases {
            let result = Email::validate_email(value);
            assert!(result.is_ok());
            assert!(!result.unwrap())
        }
    }

    #[test]
    fn test_display_success() {
        let email = "fuga@email.com";
        let result = Email::from_str(email).unwrap();
        assert_eq!(email.to_string(), result.to_string())
    }
}
