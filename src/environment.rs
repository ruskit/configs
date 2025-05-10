//! Provides environment types and utilities for environment-specific configuration.
//!
//! This module defines the `Environment` enum and related functionality for
//! determining and working with different deployment environments.

use std::{
    env,
    fmt::{Display, Formatter, Result},
};

/// # Environment
///
/// Represents the deployment environment for an application.
///
/// This enum provides variants for common deployment stages (Local, Dev, Staging, Prod)
/// and methods to determine the current environment based on environment variables.
///
/// ## Examples
///
/// ```
/// use configs::Environment;
///
/// // Get environment from RUST_ENV environment variable
/// let current_env = Environment::from_rust_env();
///
/// // Check if we're in production
/// if current_env.is_prod() {
///     println!("Running in production mode");
/// }
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Environment {
    /// Local development environment (default)
    #[default]
    Local,
    /// Development environment
    Dev,
    /// Staging/testing environment
    Staging,
    /// Production environment
    Prod,
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let printable = match *self {
            Environment::Local => "local",
            Environment::Dev => "dev",
            Environment::Staging => "stg",
            Environment::Prod => "prd",
        };
        write!(f, "{}", printable)
    }
}

impl Environment {
    /// Creates an `Environment` based on the RUST_ENV environment variable.
    ///
    /// This method checks the value of the `RUST_ENV` variable and returns
    /// the corresponding `Environment` variant. If the variable is unset or
    /// doesn't match any known environment, it defaults to `Environment::Local`.
    ///
    /// ## Returns
    ///
    /// An `Environment` value derived from the environment variable.
    pub fn from_rust_env() -> Environment {
        let env = env::var("RUST_ENV").unwrap_or_default();

        match env.as_str() {
            "production" | "PRODUCTION" | "prod" | "PROD" | "prd" | "PRD" => Environment::Prod,
            "staging" | "STAGING" | "stg" | "STG" => Environment::Staging,
            "develop" | "DEVELOP" | "dev" | "DEV" => Environment::Dev,
            _ => Environment::Local,
        }
    }

    /// Checks if the environment is Local.
    ///
    /// ## Returns
    ///
    /// `true` if the environment is Local, `false` otherwise.
    pub fn is_local(&self) -> bool {
        self == &Environment::Local
    }

    /// Checks if the environment is Dev.
    ///
    /// ## Returns
    ///
    /// `true` if the environment is Dev, `false` otherwise.
    pub fn is_dev(&self) -> bool {
        self == &Environment::Dev
    }

    /// Checks if the environment is Staging.
    ///
    /// ## Returns
    ///
    /// `true` if the environment is Staging, `false` otherwise.
    pub fn is_stg(&self) -> bool {
        self == &Environment::Staging
    }

    /// Checks if the environment is Production.
    ///
    /// ## Returns
    ///
    /// `true` if the environment is Production, `false` otherwise.
    pub fn is_prod(&self) -> bool {
        self == &Environment::Prod
    }
}
