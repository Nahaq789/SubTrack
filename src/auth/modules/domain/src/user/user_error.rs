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
}
