use std::str::FromStr;

use crate::user::{
    email::{Email, EmailError},
    password::{Password, PasswordError},
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    email: Email,
    password: Password,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthUserError {
    #[error("{0}")]
    EmailError(#[from] EmailError),

    #[error("{0}")]
    PasswordError(#[from] PasswordError),

    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Token Missing")]
    TokenMissing,

    #[error("User already exists: An account with this email address is already registered")]
    UserAlreadyExists,

    #[error("Invalid password: Password does not meet the required criteria")]
    InvalidPassword,
}

impl AuthUser {
    fn new(email: Email, password: Password) -> Self {
        Self { email, password }
    }

    pub fn build(email: &str, password: &str) -> Result<Self, AuthUserError> {
        let email = Email::from_str(email)?;
        let password = Password::from_str(password)?;

        Ok(Self::new(email, password))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_success() {
        let email = "test@email.com";
        let password = "Password123";

        let result = AuthUser::build(email, password);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(email, result.email.to_string());
        assert_eq!(password, result.password.to_string())
    }

    #[test]
    fn test_build_failed() {
        let email = "testemail.com";
        let password = "password123";

        let result = AuthUser::build(email, password);
        assert!(result.is_err());
    }
}
