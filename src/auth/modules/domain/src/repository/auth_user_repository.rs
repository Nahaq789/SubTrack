use crate::{
    auth_user::{AuthUser, AuthUserError},
    token::Token,
    user::email::Email,
};

pub trait AuthUserRepository: Send + Sync + 'static {
    fn authenticate(
        &self,
        auth: AuthUser,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Token, AuthUserError>> + Send + '_>,
    >;
    fn sign_up(
        &self,
        auth: AuthUser,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AuthUserError>> + Send + '_>>;
    fn verify_code(
        &self,
        email: Email,
        code: &str,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), AuthUserError>> + Send + '_>>;
}
