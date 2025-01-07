#[derive(Debug, Clone, PartialEq)]
pub enum UserType {
    Registerd = 1,
    Guest = 2,
}

#[derive(Debug, thiserror::Error)]
pub enum UserTypeError {
    #[error("Invalid user type value: {0}. Expected values are 1 (Registered) or 2 (Guest)")]
    InvalidValue(String),
}

impl UserType {
    pub fn from_usize(value: usize) -> Result<Self, UserTypeError> {
        match value {
            1 => Ok(UserType::Registerd),
            2 => Ok(UserType::Guest),
            _ => Err(UserTypeError::InvalidValue(value.to_string())),
        }
    }
}

impl std::str::FromStr for UserType {
    type Err = UserTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(UserType::Registerd),
            "2" => Ok(UserType::Guest),
            _ => Err(UserTypeError::InvalidValue(s.to_string())),
        }
    }
}

impl std::fmt::Display for UserType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserType::Registerd => write!(f, "1"),
            UserType::Guest => write!(f, "2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_i32_success() {
        let test_cases = vec![
            1, 2,
        ];
        for value in test_cases {
            let result = UserType::from_usize(value);
            assert!(result.is_ok())
        }
    }

    #[test]
    fn test_from_i32_failed() {
        let test_cases = vec![
            3, 10, 78, 100, 10101010,
        ];
        for value in test_cases {
            let result = UserType::from_usize(value);
            assert!(result.is_err())
        }
    }

    #[test]
    fn test_eq_variant() {
        let registerd = UserType::from_usize(1).unwrap();
        let guest = UserType::from_usize(2).unwrap();

        assert_eq!(1, registerd as usize);
        assert_eq!(2, guest as usize)
    }
}
