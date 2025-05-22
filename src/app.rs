// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines the core application configuration settings.

use super::{environment::Environment, secrets::SecretsManagerKind};

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
    ///ENV KEY: "APP_NAME"
    ///
    ///Default: default-name
    pub name: String,
    ///ENV KEY: "RUST_ENV"
    ///
    ///Default: Environment::Local
    pub env: Environment,

    ///ENV KEY: "NAMESPACE"
    ///
    ///Default: "local"
    pub namespace: String,

    ///ENV KEY: "SECRET_MANAGER"
    ///
    ///Default:false
    pub secret_manager: SecretsManagerKind,
    ///ENV KEY: "SECRET_KEY"
    ///
    ///Default: context
    pub secret_key: String,
    ///ENV KEY: "HOST_NAME"
    ///
    ///Default: 0.0.0.0
    pub host: String,
    ///ENV KEY: "APP_PORT"
    ///
    ///Default: 31033
    pub port: u64,
    ///ENV KEY: "LOG_LEVEL"
    ///
    ///Default: debug
    pub log_level: String,
    ///ENV KEY: "ENABLE_EXTERNAL_CREATES_LOGGING"
    ///
    ///Default: false
    pub enable_external_creates_logging: bool,
}

pub const APP_NAME_ENV_KEY: &str = "APP_NAME";
pub const APP_NAMESPACE_ENV_KEY: &str = "NAMESPACE";
pub const SECRET_MANAGER_ENV_KEY: &str = "SECRET_MANAGER";
pub const SECRET_KEY_ENV_KEY: &str = "SECRET_KEY";
pub const HOST_NAME_ENV_KEY: &str = "HOST_NAME";
pub const APP_PORT_ENV_KEY: &str = "APP_PORT";
pub const LOG_LEVEL_ENV_KEY: &str = "LOG_LEVEL";

impl AppConfigs {
    pub fn new() -> Self {
        let mut cfg = Self::default();

        cfg.name = std::env::var(APP_NAME_ENV_KEY).unwrap_or(cfg.name);
        cfg.env = Environment::from_rust_env();
        cfg.namespace = std::env::var(APP_NAMESPACE_ENV_KEY).unwrap_or(cfg.namespace);
        cfg.secret_manager = std::env::var(SECRET_MANAGER_ENV_KEY)
            .unwrap_or("None".into())
            .as_str()
            .try_into()
            .unwrap_or_default();
        cfg.secret_key = std::env::var(SECRET_KEY_ENV_KEY).unwrap_or(cfg.secret_key);
        cfg.host = std::env::var(HOST_NAME_ENV_KEY).unwrap_or(cfg.host);
        cfg.port = std::env::var(APP_PORT_ENV_KEY)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(cfg.port);
        cfg.log_level = std::env::var(LOG_LEVEL_ENV_KEY).unwrap_or(cfg.log_level);

        cfg
    }

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

impl Default for AppConfigs {
    fn default() -> Self {
        Self {
            name: "default-name".to_owned(),
            env: Environment::Local,
            namespace: "local".to_owned(),
            secret_manager: SecretsManagerKind::default(),
            secret_key: "context".to_owned(),
            host: "0.0.0.0".to_owned(),
            port: 31033,
            log_level: "debug".to_owned(),
            enable_external_creates_logging: false,
        }
    }
}
