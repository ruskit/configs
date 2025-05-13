// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for SQLite database connections.
//!
//! This module provides configuration options for connecting to
//! and working with SQLite databases.

/// # SqliteConfigs
///
/// Configuration structure for SQLite database connections.
///
/// This structure defines the connection parameters for SQLite databases,
/// including database file path and (optional) credentials.
///
/// ## Examples
///
/// ```
/// use configs::SqliteConfigs;
///
/// let mut sqlite_config = SqliteConfigs::default();
/// sqlite_config.file = "/path/to/database.db".to_string();
/// ```
#[derive(Debug, Clone, Default)]
pub struct SqliteConfigs {
    /// ENV KEY: "SQLITE_FILE_NAME"
    ///
    /// The SQLite database file path (Default: "local.db")
    pub file: String,
}

pub const SQLITE_FILE_NAME_ENV_KEY: &str = "SQLITE_FILE_NAME";

impl SqliteConfigs {
    /// Creates a new `SqliteConfigs` instance from environment variables.
    ///
    /// This method initializes the SQLite configuration from the environment variable
    /// for the database file path.
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.file = std::env::var(SQLITE_FILE_NAME_ENV_KEY).unwrap_or(cfgs.file);

        cfgs
    }
}
