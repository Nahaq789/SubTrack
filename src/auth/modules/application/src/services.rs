pub mod user_service;

pub trait AuthService {}

pub trait UserService: Send + Sync {
    fn create_user(
        &self, user: domain::user::User,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>>;

    fn update(
        &self, user: domain::user::User,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>>;
}
