// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for RabbitMQ message broker.
//!
//! This module provides configuration options for connecting to
//! and working with RabbitMQ message brokers.

/// # RabbitMQConfigs
///
/// Configuration structure for RabbitMQ connections.
///
/// This structure defines the connection parameters for RabbitMQ message brokers,
/// including host, port, credentials, and virtual host.
///
/// ## Examples
///
/// ```
/// use configs::RabbitMQConfigs;
///
/// let mut rabbitmq_config = RabbitMQConfigs::default();
/// rabbitmq_config.host = "rabbitmq.example.com".to_string();
/// rabbitmq_config.vhost = "/production".to_string();
/// ```
#[derive(Debug, Clone)]
pub struct RabbitMQConfigs {
    /// ENV KEY: "RABBITMQ_HOST"
    ///
    /// The RabbitMQ server host (Default: "localhost")
    pub host: String,
    /// ENV KEY: "RABBITMQ_PORT"
    ///
    /// The RabbitMQ server port (Default: 5672)
    pub port: u64,
    /// ENV KEY: "RABBITMQ_USER"
    ///
    /// The RabbitMQ username (Default: "default")
    pub user: String,
    /// ENV KEY: "RABBITMQ_PASSWORD"
    ///
    /// The RabbitMQ password (Default: "default")
    pub password: String,
    /// ENV KEY: "RABBITMQ_VHOST"
    ///
    /// The RabbitMQ virtual host (Default: "")
    pub vhost: String,
}

pub const RABBITMQ_HOST_ENV_KEY: &str = "RABBITMQ_HOST";
pub const RABBITMQ_PORT_ENV_KEY: &str = "RABBITMQ_PORT";
pub const RABBITMQ_USER_ENV_KEY: &str = "RABBITMQ_USER";
pub const RABBITMQ_PASSWORD_ENV_KEY: &str = "RABBITMQ_PASSWORD";
pub const RABBITMQ_VHOST_ENV_KEY: &str = "RABBITMQ_VHOST";

impl RabbitMQConfigs {
    /// Creates a new `RabbitMQConfigs` instance from environment variables.
    ///
    /// This method initializes the RabbitMQ configuration from environment variables
    /// for host, port, user, password, and virtual host.
    ///
    /// ## Returns
    ///
    /// A new `RabbitMQConfigs` from environment variables.
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.host = std::env::var(RABBITMQ_HOST_ENV_KEY).unwrap_or(cfgs.host);
        cfgs.port = std::env::var(RABBITMQ_PORT_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(cfgs.port);
        cfgs.user = std::env::var(RABBITMQ_USER_ENV_KEY).unwrap_or(cfgs.user);
        cfgs.password = std::env::var(RABBITMQ_PASSWORD_ENV_KEY).unwrap_or(cfgs.password);
        cfgs.vhost = std::env::var(RABBITMQ_VHOST_ENV_KEY).unwrap_or(cfgs.vhost);

        cfgs
    }
}

impl Default for RabbitMQConfigs {
    fn default() -> Self {
        Self {
            host: "localhost".to_owned(),
            port: 5672,
            user: "default".to_owned(),
            password: "default".to_owned(),
            vhost: Default::default(),
        }
    }
}
