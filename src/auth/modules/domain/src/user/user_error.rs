use thiserror::Error;

use crate::EntityIdError;

use super::{email::EmailError, name::NameError, user_type::UserTypeError};

#[derive(Debug, Error)]
pub enum UserError {
    #[error("{0}")]
    EmailError(#[from] EmailError),

    #[error("{0}")]
    NameError(#[from] NameError),

    #[error("{0}")]
    EntityIdError(#[from] EntityIdError),

    #[error("{0}")]
    UserTypeError(#[from] UserTypeError),

    #[error("Failed to find by id user: {0}")]
    FindByIdError(String),

    #[error("Failed to create user: {0}")]
    CreateUserError(String),

    #[error("Failed to update user: {0}")]
    UpdateUserError(String),
}
