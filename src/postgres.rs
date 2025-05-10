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
    /// The PostgreSQL server host (Default: "localhost")
    pub host: String,
    /// The PostgreSQL username (Default: "")
    pub user: String,
    /// The PostgreSQL password (Default: "")
    pub password: String,
    /// The PostgreSQL server port (Default: 0)
    pub port: u16,
    /// The PostgreSQL database name (Default: "")
    pub db: String,
    /// The SSL mode for the connection (Default: PostgresSslMode::Disabled)
    pub ssl_mode: PostgresSslMode,
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
