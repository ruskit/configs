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
    /// ENV KEY: "IDENTITY_SERVER_URL"
    ///
    /// Identity Server URL (Default: "")
    pub url: String,
    /// ENV KEY: "IDENTITY_SERVER_REALM"
    ///
    /// Identity Application Realm (Default: "")
    ///
    /// In Auth0, Realm is the same as Domain
    pub realm: String,
    /// ENV KEY: "IDENTITY_SERVER_AUDIENCE"
    ///
    /// OAuth audience value (Default: "")
    pub audience: String,
    /// ENV KEY: "IDENTITY_SERVER_ISSUER"
    ///
    /// OAuth token issuer (Default: "")
    pub issuer: String,
    /// ENV KEY: "IDENTITY_SERVER_CLIENT_ID"
    ///
    /// OAuth client ID (Default: "")
    pub client_id: String,
    /// ENV KEY: "IDENTITY_SERVER_CLIENT_SECRET"
    ///
    /// OAuth client secret (Default: "")
    pub client_secret: String,
    /// ENV KEY: "IDENTITY_SERVER_GRANT_TYPE"
    ///
    /// OAuth grant type (Default: "client_credentials")
    pub grant_type: String,
}

pub const IDENTITY_SERVER_URL_ENV_KEY: &str = "IDENTITY_SERVER_URL";
pub const IDENTITY_SERVER_REALM_ENV_KEY: &str = "IDENTITY_SERVER_REALM";
pub const IDENTITY_SERVER_AUDIENCE_ENV_KEY: &str = "IDENTITY_SERVER_AUDIENCE";
pub const IDENTITY_SERVER_ISSUER_ENV_KEY: &str = "IDENTITY_SERVER_ISSUER";
pub const IDENTITY_SERVER_GRANT_TYPE_ENV_KEY: &str = "IDENTITY_SERVER_GRANT_TYPE";
pub const IDENTITY_SERVER_CLIENT_ID_ENV_KEY: &str = "IDENTITY_SERVER_CLIENT_ID";
pub const IDENTITY_SERVER_CLIENT_SECRET_ENV_KEY: &str = "IDENTITY_SERVER_CLIENT_SECRET";

impl IdentityServerConfigs {
    /// Creates a new `IdentityServerConfigs` instance from environment variables.
    ///
    /// This method initializes the identity server configuration from environment variables
    /// for URL, realm, audience, issuer, client ID, client secret, and grant type.
    ///
    /// ## Returns
    ///
    /// A new `IdentityServerConfigs` from environment variables.
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.url = std::env::var(IDENTITY_SERVER_URL_ENV_KEY).unwrap_or(cfgs.url);
        cfgs.realm = std::env::var(IDENTITY_SERVER_REALM_ENV_KEY).unwrap_or(cfgs.realm);
        cfgs.audience = std::env::var(IDENTITY_SERVER_AUDIENCE_ENV_KEY).unwrap_or(cfgs.audience);
        cfgs.issuer = std::env::var(IDENTITY_SERVER_ISSUER_ENV_KEY).unwrap_or(cfgs.issuer);
        cfgs.client_id = std::env::var(IDENTITY_SERVER_CLIENT_ID_ENV_KEY).unwrap_or(cfgs.client_id);
        cfgs.client_secret =
            std::env::var(IDENTITY_SERVER_CLIENT_SECRET_ENV_KEY).unwrap_or(cfgs.client_secret);
        cfgs.grant_type =
            std::env::var(IDENTITY_SERVER_GRANT_TYPE_ENV_KEY).unwrap_or(cfgs.grant_type);

        cfgs
    }
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
