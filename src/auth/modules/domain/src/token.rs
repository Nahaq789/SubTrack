#[derive(Debug)]
pub struct Token {
    jwt: String,
    refresh: String,
}

impl Token {
    pub fn new(jwt: String, refresh: String) -> Self {
        Self { jwt, refresh }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    #[test]
    fn test_token_create_success() {
        let result = Token::new("jwt_token".to_string(), "refresh_token".to_string());

        assert_eq!(result.jwt, "jwt_token".to_string());
        assert_eq!(result.refresh, "refresh_token".to_string())
    }
}
