pub mod auth_user;
pub mod repository;
pub mod token;
pub mod user;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum EntityIdError {
    #[error("It is not in the prefix_UUID format.")]
    InvalidFormat,

    #[error("Invalid UUID format")]
    InvalidUuid,
}

pub trait EntityId {
    fn generate_id(p: &str, u: Option<uuid::Uuid>) -> String;
    fn type_name(&self) -> &String;
    fn value(&self) -> &String;
}
