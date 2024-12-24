use thiserror::Error;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

#[derive(Debug)]
pub struct ApiSettings {
    pub host: String,
    pub port: String,
}

#[derive(Debug)]
pub struct AwsSettings {
    auth: String,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SettingsError {
    #[error("Cannot load env. key: {0}")]
    InvalidLoadConfig(String),

    #[error("State build error: {0}")]
    StateBuildError(String),
}

impl ApiSettings {
    pub fn build() -> Result<Self, SettingsError> {
        let host = std::env::var("HOST")
            .map_err(|_| SettingsError::InvalidLoadConfig("HOST".to_string()))?;
        let port = std::env::var("PORT")
            .map_err(|_| SettingsError::InvalidLoadConfig("PORT".to_string()))?;

        Ok(Self { host, port })
    }
}

impl AwsSettings {
    pub fn build() -> Result<Self, SettingsError> {
        let auth = std::env::var("AUTH_TABLE")
            .map_err(|_| SettingsError::InvalidLoadConfig("AUTH_TABLE".to_string()))?;

        Ok(Self { auth })
    }
}

pub fn set_up_tracing_subscriber() {
    const CREDENTIALS: &str = "credentials";
    let filter = EnvFilter::from_default_env();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_target(true)
                .with_ansi(false)
                .with_filter(filter)
                .with_filter(filter::filter_fn(|metadata| {
                    if metadata.target().contains(CREDENTIALS) {
                        false
                    } else {
                        true
                    }
                })),
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    const HOST: &str = "1.1.1.1";
    const PORT: &str = "7878";
    const AUTH_TABLE: &str = "payment";

    fn clear_env() {
        std::env::remove_var("HOST");
        std::env::remove_var("PORT");
        std::env::remove_var("AUTH_TABLE");
    }

    #[test]
    fn test_api_settings_build_success() {
        clear_env();
        std::env::set_var("HOST", HOST);
        std::env::set_var("PORT", PORT);

        let result = ApiSettings::build();

        assert!(&result.is_ok());
        let result = result.unwrap();
        assert_eq!(&result.host, HOST);
        assert_eq!(&result.port, PORT)
    }

    #[test]
    fn test_api_settings_build_host_failed() {
        clear_env();
        let result = ApiSettings::build();

        assert!(result.is_err());
        assert_eq!(
            SettingsError::InvalidLoadConfig("HOST".to_string()),
            result.unwrap_err()
        )
    }

    #[test]
    fn test_api_settings_build_port_failed() {
        clear_env();
        std::env::set_var("HOST", HOST);
        let result = ApiSettings::build();

        assert!(result.is_err());
        assert_eq!(
            SettingsError::InvalidLoadConfig("PORT".to_string()),
            result.unwrap_err()
        )
    }

    #[test]
    fn aws_settings_build_payment_success() {
        clear_env();
        std::env::set_var("AUTH_TABLE", AUTH_TABLE);
        let result = AwsSettings::build();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(&result.auth, AUTH_TABLE)
    }

    #[test]
    fn aws_settings_build_payment_failed() {
        clear_env();
        let result = AwsSettings::build();

        assert!(result.is_err());
        assert_eq!(
            SettingsError::InvalidLoadConfig("AUTH_TABLE".to_string()),
            result.unwrap_err()
        )
    }
}
