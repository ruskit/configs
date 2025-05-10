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
    /// The SQLite database file path (Default: "local.db")
    pub file: String,
    /// The SQLite username (Default: "postgres")
    pub user: String,
    /// The SQLite password (Default: "postgres")
    pub password: String,
}
