// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for identity server integration.
//!
//! This module provides configuration options for connecting to and
//! authenticating with identity servers like Auth0, Keycloak, etc.

/// # IdentityServerConfigs
///
/// Configuration structure for identity server integration.
///
/// This structure defines the parameters needed to connect to and authenticate
/// with an identity provider like Auth0, Keycloak, or other OAuth/OpenID Connect
/// compatible identity servers.
///
/// ## Examples
///
/// ```
/// use configs::IdentityServerConfigs;
///
/// let identity_config = IdentityServerConfigs::default();
/// // Configure with actual values before use
/// ```
#[derive(Debug, Clone)]
pub struct IdentityServerConfigs {
    /// Identity Server URL (Default: "")
    pub url: String,
    /// Identity Application Realm (Default: "")
    ///
    /// In Auth0, Realm is the same as Domain
    pub realm: String,
    /// OAuth audience value (Default: "")
    pub audience: String,
    /// OAuth token issuer (Default: "")
    pub issuer: String,
    /// OAuth client ID (Default: "")
    pub client_id: String,
    /// OAuth client secret (Default: "")
    pub client_secret: String,
    /// OAuth grant type (Default: "client_credentials")
    pub grant_type: String,
}

impl Default for IdentityServerConfigs {
    fn default() -> Self {
        Self {
            url: Default::default(),
            realm: Default::default(),
            audience: Default::default(),
            issuer: Default::default(),
            client_id: Default::default(),
            client_secret: Default::default(),
            grant_type: "client_credentials".to_owned(),
        }
    }
}
