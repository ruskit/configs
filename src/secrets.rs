// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines types for secret management configuration.
//!
//! This module provides enums and utilities for configuring secret management
//! backends to be used by applications.

/// # SecretsManagerKind
///
/// Enum representing the type of secrets management service to use.
///
/// This enum allows applications to configure which secrets management service
/// they want to use, or to disable secrets management entirely.
///
/// ## Variants
///
/// * `None` - No secrets manager is used (default)
/// * `AWSSecretManager` - AWS Secrets Manager service is used
///
/// ## Examples
///
/// ```
/// use configs::SecretsManagerKind;
///
/// // Get a SecretsManagerKind from a string configuration
/// let kind = SecretsManagerKind::from("AWS");
///
/// // Default to no secrets manager
/// let default_kind = SecretsManagerKind::default();
/// ```
#[derive(Debug, Clone, Default)]
pub enum SecretsManagerKind {
    /// No secrets management (default)
    #[default]
    None,
    /// AWS Secrets Manager service
    AWSSecretManager,
}

impl From<&str> for SecretsManagerKind {
    /// Creates a `SecretsManagerKind` from a string slice.
    ///
    /// The conversion is case-insensitive. If the input string does not match any
    /// known secrets manager kind, it defaults to `SecretsManagerKind::None`.
    ///
    /// ## Parameters
    ///
    /// * `value` - A string slice containing the name of the secrets manager
    ///
    /// ## Returns
    ///
    /// A `SecretsManagerKind` variant corresponding to the input string, or
    /// `SecretsManagerKind::None` if no match is found.
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "AWS" => SecretsManagerKind::AWSSecretManager,
            _ => SecretsManagerKind::None,
        }
    }
}

impl From<&String> for SecretsManagerKind {
    /// Creates a `SecretsManagerKind` from a reference to a String.
    ///
    /// The conversion is case-insensitive. If the input string does not match any
    /// known secrets manager kind, it defaults to `SecretsManagerKind::None`.
    ///
    /// ## Parameters
    ///
    /// * `value` - A reference to a String containing the name of the secrets manager
    ///
    /// ## Returns
    ///
    /// A `SecretsManagerKind` variant corresponding to the input string, or
    /// `SecretsManagerKind::None` if no match is found.
    fn from(value: &String) -> Self {
        match value.to_uppercase().as_str() {
            "AWS" => SecretsManagerKind::AWSSecretManager,
            _ => SecretsManagerKind::None,
        }
    }
}
