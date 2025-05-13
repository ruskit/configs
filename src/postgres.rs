// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for PostgreSQL database connections.
//!
//! This module provides configuration options for connecting to
//! and working with PostgreSQL databases.

/// # PostgresConfigs
///
/// Configuration structure for PostgreSQL database connections.
///
/// This structure defines the connection parameters for PostgreSQL databases,
/// including host, port, credentials, database name, and SSL configuration.
///
/// ## Examples
///
/// ```
/// use configs::{PostgresConfigs, PostgresSslMode};
///
/// let mut pg_config = PostgresConfigs::default();
/// pg_config.host = "db.example.com".to_string();
/// pg_config.ssl_mode = PostgresSslMode::Required;
/// ```
#[derive(Debug, Clone)]
pub struct PostgresConfigs {
    /// ENV KEY: "POSTGRES_HOST"
    ///
    /// The PostgreSQL server host (Default: "localhost")
    pub host: String,
    /// ENV KEY: "POSTGRES_USER"
    ///
    /// The PostgreSQL username (Default: "")
    pub user: String,
    /// ENV KEY: "POSTGRES_PASSWORD"
    ///
    /// The PostgreSQL password (Default: "")
    pub password: String,
    /// ENV KEY: "POSTGRES_PORT"
    ///
    /// The PostgreSQL server port (Default: 0)
    pub port: u16,
    /// ENV KEY : "POSTGRES_DB"
    ///
    /// The PostgreSQL database name (Default: "")
    pub db: String,
    /// ENV KEY: "POSTGRES_SSL_MODE"
    ///
    /// The SSL mode for the connection (Default: PostgresSslMode::Disabled)
    pub ssl_mode: PostgresSslMode,
    /// ENV KEY: "POSTGRES_CA_PATH"
    ///
    /// Path to CA certificate for SSL verification (Default: "")
    pub ca_path: String,
}

/// # PostgresSslMode
///
/// Enum representing PostgreSQL SSL connection modes.
///
/// This enum defines how SSL is used in PostgreSQL connections.
///
/// ## Variants
///
/// * `Disabled` - Don't use SSL (default)
/// * `Required` - Always use SSL/TLS
///
/// ## Examples
///
/// ```
/// use configs::PostgresSslMode;
///
/// let ssl_mode = PostgresSslMode::from("required".to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PostgresSslMode {
    /// Don't use SSL (default)
    #[default]
    Disabled,
    /// Always use SSL/TLS
    Required,
}

impl From<String> for PostgresSslMode {
    /// Creates a `PostgresSslMode` from a String.
    ///
    /// The conversion is case-sensitive. If the input string equals "required",
    /// it returns `PostgresSslMode::Required`, otherwise it returns `PostgresSslMode::Disabled`.
    ///
    /// ## Parameters
    ///
    /// * `value` - A String containing the SSL mode
    ///
    /// ## Returns
    ///
    /// A `PostgresSslMode` variant corresponding to the input string
    fn from(value: String) -> Self {
        if value.eq("required") {
            return Self::Required;
        }

        Self::Disabled
    }
}

pub const POSTGRES_HOST_ENV_KEY: &str = "POSTGRES_HOST";
pub const POSTGRES_PORT_ENV_KEY: &str = "POSTGRES_PORT";
pub const POSTGRES_USER_ENV_KEY: &str = "POSTGRES_USER";
pub const POSTGRES_PASSWORD_ENV_KEY: &str = "POSTGRES_PASSWORD";
pub const POSTGRES_DB_ENV_KEY: &str = "POSTGRES_DB";
pub const POSTGRES_SSL_MODE_ENV_KEY: &str = "POSTGRES_SSL_MODE";
pub const POSTGRES_CA_PATH_ENV_KEY: &str = "POSTGRES_CA_PATH";

impl PostgresConfigs {
    /// Creates a new `PostgresConfigs` with environment variables.
    ///
    /// This method initializes the PostgreSQL configuration with environment variables
    /// for the host, port, user, password, database name, SSL mode, and CA path.
    ///
    /// ## Returns
    ///
    /// A new `PostgresConfigs` with environment variables.
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.host = std::env::var(POSTGRES_HOST_ENV_KEY).unwrap_or(cfgs.host);
        cfgs.port = std::env::var(POSTGRES_PORT_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(cfgs.port);
        cfgs.user = std::env::var(POSTGRES_USER_ENV_KEY).unwrap_or(cfgs.user);
        cfgs.password = std::env::var(POSTGRES_PASSWORD_ENV_KEY).unwrap_or(cfgs.password);
        cfgs.db = std::env::var(POSTGRES_DB_ENV_KEY).unwrap_or(cfgs.db);
        cfgs.ssl_mode = std::env::var(POSTGRES_SSL_MODE_ENV_KEY)
            .ok()
            .map(|v| PostgresSslMode::from(v))
            .unwrap_or(cfgs.ssl_mode);
        cfgs.ca_path = std::env::var(POSTGRES_CA_PATH_ENV_KEY).unwrap_or(cfgs.ca_path);

        cfgs
    }
}

impl Default for PostgresConfigs {
    fn default() -> Self {
        Self {
            host: "localhost".to_owned(),
            user: Default::default(),
            password: Default::default(),
            port: Default::default(),
            db: Default::default(),
            ssl_mode: Default::default(),
            ca_path: Default::default(),
        }
    }
}
