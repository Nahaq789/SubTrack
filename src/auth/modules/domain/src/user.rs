pub mod email;
pub mod name;
pub mod password;
pub mod user_error;
pub mod user_id;
pub mod user_type;

use std::str::FromStr;

use email::Email;
use name::Name;
use user_error::UserError;
use user_id::UserId;
use user_type::UserType;

#[derive(Debug, Clone)]
pub struct User {
    user_id: UserId,
    email: Email,
    name: Name,
    user_type: UserType,
    profile_icon_path: Option<String>,
}

impl User {
    fn new(
        user_id: UserId,
        email: Email,
        name: Name,
        user_type: UserType,
        profile_icon_path: Option<String>,
    ) -> Self {
        Self {
            user_id,
            email,
            name,
            user_type,
            profile_icon_path,
        }
    }

    pub fn build(
        user_id: &str,
        email: &str,
        name: &str,
        user_type: usize,
        profile_icon_path: Option<String>,
    ) -> Result<Self, UserError> {
        let user_id = UserId::from_str(user_id)?;
        let email = Email::from_str(email)?;
        let name = Name::from_str(name)?;
        let user_type = UserType::from_usize(user_type)?;

        Ok(Self {
            user_id,
            email,
            name,
            user_type,
            profile_icon_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_user_build_success() {
        let user_id = "usr_123e4567-e89b-12d3-a456-426614174000";
        let email = "test@example.com";
        let name = "Test User";
        let user_type = 1;
        let profile_icon = Some("avatar.jpg".to_string());

        let user = User::build(user_id, email, name, user_type, profile_icon.clone()).unwrap();

        assert_eq!(user.user_id.to_string(), user_id);
        assert_eq!(user.email.to_string(), email);
        assert_eq!(user.name.to_string(), name);
        assert_eq!(user.user_type as usize, user_type);
        assert_eq!(user.profile_icon_path, profile_icon);
    }

    #[test]
    fn test_user_build_without_icon() {
        let user = User::build(
            "usr_123e4567-e89b-12d3-a456-426614174000",
            "test@example.com",
            "Test User",
            1,
            None,
        )
        .unwrap();
        assert!(user.profile_icon_path.is_none());
    }

    #[test]
    fn test_user_build_empty_name() {
        let result = User::build(
            "usr_123e4567-e89b-12d3-a456-426614174000",
            "test@example.com",
            "",
            1,
            None,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_user_build_empty_email() {
        let result = User::build(
            "usr_123e4567-e89b-12d3-a456-426614174000",
            "",
            "Test User",
            1,
            None,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_user_build_zero_user_type() {
        let result = User::build(
            "usr_123e4567-e89b-12d3-a456-426614174000",
            "test@example.com",
            "Test User",
            0,
            None,
        );
        assert!(result.is_err());
    }
}
