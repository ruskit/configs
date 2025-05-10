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
    /// The RabbitMQ server host (Default: "localhost")
    pub host: String,
    /// The RabbitMQ server port (Default: 5672)
    pub port: u64,
    /// The RabbitMQ username (Default: "default")
    pub user: String,
    /// The RabbitMQ password (Default: "default")
    pub password: String,
    /// The RabbitMQ virtual host (Default: "")
    pub vhost: String,
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
