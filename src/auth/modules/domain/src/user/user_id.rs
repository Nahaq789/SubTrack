use std::{fmt::Display, str::FromStr};

use uuid::Uuid;

use crate::{EntityId, EntityIdError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId {
    prefix: String,
    value: String,
}

const USER_PREFIX: &str = "usr";

impl UserId {
    pub fn new() -> Self {
        let value = Self::generate_id(USER_PREFIX, None);
        Self {
            prefix: USER_PREFIX.to_owned(),
            value,
        }
    }
}

impl EntityId for UserId {
    fn generate_id(p: &str, u: Option<uuid::Uuid>) -> String {
        match u {
            Some(u) => format!("{}_{}", p, u),
            None => {
                let value = Uuid::new_v4();
                format!("{}_{}", p, value)
            }
        }
    }
    fn type_name(&self) -> &String {
        &self.prefix
    }

    fn value(&self) -> &String {
        &self.value
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self {
            prefix: USER_PREFIX.to_owned(),
            value: Self::generate_id(USER_PREFIX, Some(value)),
        }
    }
}

impl FromStr for UserId {
    type Err = EntityIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: Vec<&str> = s.split("_").collect();
        if value.len() != 2 {
            return Err(EntityIdError::InvalidFormat);
        }
        if value[0] != USER_PREFIX {
            return Err(EntityIdError::InvalidFormat);
        }
        let uuid = Uuid::parse_str(value[1]).map_err(|_| EntityIdError::InvalidUuid)?;
        Ok(Self::from(uuid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_success() {
        let id = UserId::new();
        assert!(!id.value.is_empty());
        assert!(id.prefix.starts_with("usr"))
    }

    #[test]
    fn test_type_name_success() {
        let id = UserId::new();
        assert!(!id.type_name().is_empty());
        assert_eq!(USER_PREFIX, id.type_name());
    }

    #[test]
    fn test_from_success() {
        let uuid = Uuid::new_v4();
        let format = format!("usr_{}", uuid);
        let result = UserId::from(uuid);
        assert!(result.value.starts_with("usr_"));
        assert_eq!(format, result.value);
    }

    #[test]
    fn test_from_str_success() {
        let uuid = Uuid::new_v4();
        let format = format!("usr_{}", uuid);
        let result = UserId::from_str(&format);
        assert!(result.is_ok());
        assert_eq!(format, result.unwrap().value);
    }

    #[test]
    fn test_display_format() {
        let id = UserId::new();
        let display_string = id.to_string();
        assert!(display_string.starts_with("usr_"));
    }

    #[test]
    fn test_clone_equality() {
        let id1 = UserId::new();
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_debug_format() {
        let id = UserId::new();
        let debug_string = format!("{:?}", id);
        assert!(!debug_string.is_empty());
    }

    #[test]
    fn test_from_str_failed_invalid_format() {
        let result = UserId::from_str("invalid");
        assert!(matches!(result, Err(EntityIdError::InvalidFormat)));
    }

    #[test]
    fn test_from_str_failed_invalid_uuid() {
        let format = format!("usr_{}", "invalid-uuid");
        let result = UserId::from_str(&format);
        assert!(matches!(result, Err(EntityIdError::InvalidUuid)));
    }

    #[test]
    fn test_from_str_failed_wrong_prefix() {
        let uuid = Uuid::new_v4();
        let format = format!("wrong_{}", uuid);
        let result = UserId::from_str(&format);
        assert!(matches!(result, Err(EntityIdError::InvalidFormat)));
    }

    #[test]
    fn test_value_reference() {
        let id = UserId::new();
        let value_ref = id.value();
        assert_eq!(&id.value, value_ref);
    }

    #[test]
    fn test_multiple_instances_unique() {
        let id1 = UserId::new();
        let id2 = UserId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_from_str_empty_string() {
        let result = UserId::from_str("");
        assert!(matches!(result, Err(EntityIdError::InvalidFormat)));
    }

    #[test]
    fn test_from_str_too_many_parts() {
        let result = UserId::from_str("usr_uuid_extra");
        assert!(matches!(result, Err(EntityIdError::InvalidFormat)));
    }

    #[test]
    fn test_display_and_from_str_consistency() {
        let original = UserId::new();
        let display_string = original.to_string();
        assert!(display_string.contains(&original.value));
    }
}
