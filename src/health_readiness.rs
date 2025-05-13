// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for health and readiness check endpoints.
//!
//! This module provides configuration options for health and readiness check endpoints
//! that can be used for Kubernetes probes or other monitoring systems.

/// # HealthReadinessConfigs
///
/// Configuration for health and readiness check HTTP endpoints.
///
/// This struct defines configuration parameters for a health and readiness
/// HTTP server that can be used with Kubernetes probes or other monitoring systems.
///
/// ## Examples
///
/// ```
/// use configs::HealthReadinessConfigs;
///
/// let config = HealthReadinessConfigs::default();
/// if config.enable {
///     println!("Health and readiness server will listen on: {}", config.health_readiness_addr());
/// }
/// ```
#[derive(Debug, Clone)]
pub struct HealthReadinessConfigs {
    /// ENV KEY: "HEALTH_READINESS_PORT"
    ///
    /// The port to listen on for health and readiness check requests (Default: 8888)
    pub port: u64,
    /// ENV KEY: "ENABLE_HEALTH_READINESS"
    ///
    /// Whether the health and readiness check server should be enabled (Default: false)
    pub enable: bool,
}

pub const HEALTH_READINESS_PORT_ENV_KEY: &str = "HEALTH_READINESS_PORT";
pub const ENABLE_HEALTH_READINESS_ENV_KEY: &str = "ENABLE_HEALTH_READINESS";

impl HealthReadinessConfigs {
    /// Creates a new `HealthReadinessConfigs` with environments variables.
    ///
    /// This method initializes the health and readiness configuration with environments variables
    /// for the port and enable flag.
    ///
    /// ## Returns
    ///
    /// A new `HealthReadinessConfigs` with environments variables.
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.port = std::env::var(HEALTH_READINESS_PORT_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(cfgs.port);
        cfgs.enable = std::env::var(ENABLE_HEALTH_READINESS_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(cfgs.enable);

        cfgs
    }
}

impl Default for HealthReadinessConfigs {
    fn default() -> Self {
        Self {
            port: 8888,
            enable: false,
        }
    }
}

impl HealthReadinessConfigs {
    /// Returns the formatted address string for the health and readiness server.
    ///
    /// ## Returns
    ///
    /// A String containing the formatted address (e.g., "0.0.0.0:8888").
    pub fn health_readiness_addr(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}
