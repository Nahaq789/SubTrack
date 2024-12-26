use std::str::FromStr;

use crate::user::{
    email::{Email, EmailError},
    password::{Password, PasswordError},
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    email: Email,
    password: Password,
    verify_code: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthUserError {
    #[error("{0}")]
    EmailError(#[from] EmailError),

    #[error("{0}")]
    PasswordError(#[from] PasswordError),
}

impl AuthUser {
    fn new(email: Email, password: Password, verify_code: Option<String>) -> Self {
        Self {
            email,
            password,
            verify_code,
        }
    }

    pub fn build(
        email: &str,
        password: &str,
        verify_code: Option<String>,
    ) -> Result<Self, AuthUserError> {
        let email = Email::from_str(email)?;
        let password = Password::from_str(password)?;

        Ok(Self::new(email, password, verify_code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_success() {
        let email = "test@email.com";
        let password = "Password123";

        let result = AuthUser::build(email, password, Some("123456".to_string()));
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(email, result.email.to_string());
        assert_eq!(password, result.password.to_string())
    }

    #[test]
    fn test_build_failed() {
        let email = "testemail.com";
        let password = "password123";

        let result = AuthUser::build(email, password, None);
        assert!(result.is_err());
    }
}
