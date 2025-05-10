//! Defines the core application configuration settings.

use crate::{Environment, SecretsManagerKind};

/// # AppConfigs
///
/// Configuration structure for basic application settings.
///
/// This structure defines the core configuration parameters that most applications need,
/// including application name, environment, host, port, and logging settings.
///
/// ## Example
///
/// ```
/// use configs::AppConfigs;
///
/// let app_config = AppConfigs::default();
/// println!("Application will listen on: {}", app_config.app_addr());
/// ```
#[derive(Debug, Clone)]
pub struct AppConfigs {
    ///Default: APP_NAME
    pub name: String,
    ///Default: Environment::Local
    pub env: Environment,
    ///Default:false
    pub secret_manager: SecretsManagerKind,
    ///Default: context
    pub secret_key: String,
    ///Default: 0.0.0.0
    pub host: String,
    ///Default: 31033
    pub port: u64,
    ///Default: debug
    pub log_level: String,
    ///Default: false
    pub enable_external_creates_logging: bool,
}

impl Default for AppConfigs {
    fn default() -> Self {
        Self {
            name: "APP_NAME".to_owned(),
            env: Environment::Local,
            secret_manager: SecretsManagerKind::default(),
            secret_key: "context".to_owned(),
            host: "0.0.0.0".to_owned(),
            port: 31033,
            log_level: "debug".to_owned(),
            enable_external_creates_logging: false,
        }
    }
}

impl AppConfigs {
    /// Returns the formatted address string in the format "host:port".
    ///
    /// This is a convenience method for getting the complete bind address
    /// for the application server.
    ///
    /// ## Returns
    ///
    /// A String containing the formatted address (e.g., "0.0.0.0:31033").
    pub fn app_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
