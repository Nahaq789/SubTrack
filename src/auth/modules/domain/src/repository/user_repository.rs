use crate::user::{user_error::UserError, User};

pub trait UserRepository: Send + Sync {
    fn find_by_id(
        &self,
        id: i32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<User, UserError>> + Send + '_>>;
    fn create(
        &self,
        user: User,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), UserError>> + Send + '_>>;
    fn update(
        &self,
        user: User,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), UserError>> + Send + '_>>;
}
