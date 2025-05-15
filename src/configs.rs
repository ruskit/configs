// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Provides the central configuration aggregation structures and traits.
//!
//! This module defines the main `Configs` structure that aggregates all service-specific
//! configurations, as well as the `DynamicConfigs` trait for extensibility.
//!
//! ## Architecture
//!
//! The configuration system is designed around these key concepts:
//!
//! 1. **Centralized access**: A single `Configs<T>` structure contains all configuration
//! 2. **Service-specific modules**: Each service has its own configuration module
//! 3. **Environment variable integration**: Automatic loading of values from environment
//! 4. **Extensibility**: Custom application configuration via the `DynamicConfigs` trait
//!
//! ## In-depth Example
//!
//! ```rust
//! use configs::{Configs, DynamicConfigs};
//! use std::env;
//!
//! // Create custom application configuration
//! #[derive(Debug, Clone, Default)]
//! struct MyAppConfig {
//!     feature_flag_enabled: bool,
//!     user_limit: u32,
//! }
//!
//! impl DynamicConfigs for MyAppConfig {
//!     fn load(&mut self) {
//!         self.feature_flag_enabled = env::var("MY_APP_FEATURE_FLAG")
//!             .map(|v| v.to_lowercase() == "true")
//!             .unwrap_or(false);
//!
//!         self.user_limit = env::var("MY_APP_USER_LIMIT")
//!             .ok()
//!             .and_then(|s| s.parse().ok())
//!             .unwrap_or(100);
//!     }
//! }
//!
//! // Create and use the configuration
//! let mut config = Configs::<MyAppConfig>::new();
//!
//! // Application logic using various configuration components
//! if config.app.env.is_dev() {
//!     println!("Running in development mode");
//! }
//!
//! if config.dynamic.feature_flag_enabled {
//!     println!("Special feature is enabled");
//! }
//!
//! // Use database configuration
//! println!("Will connect to PostgreSQL at {}", config.postgres.host);
//!
//! // Use message broker configuration
//! println!("RabbitMQ connection at {}", config.rabbitmq.uri());
//! ```

use crate::otlp;

use super::{
    app, aws, dynamic::DynamicConfigs, dynamo, health_readiness, identity_server, influx, kafka,
    mqtt, postgres, rabbitmq, sqlite,
};

/// # Configs
///
/// The central configuration container that aggregates all service configurations.
///
/// This generic structure holds configuration objects for all supported services and components,
/// providing a single point of access for application configuration. The generic parameter `T`
/// allows for extension with application-specific configuration via the `DynamicConfigs` trait.
///
/// ## Type Parameters
///
/// * `T` - A type implementing `DynamicConfigs` for application-specific configuration
///
/// ## Examples
///
/// ```
/// use configs::{Configs, Empty};
///
/// // Create a configuration with no dynamic configs
/// let config = Configs::<Empty>::default();
///
/// ```
#[derive(Debug, Clone, Default)]
pub struct Configs<T: DynamicConfigs> {
    /// Core application configuration
    pub app: app::AppConfigs,
    /// OTLP (OpenTelemetry) configuration
    pub otlp: otlp::OTLPConfigs,
    /// Identity server configuration
    pub identity: identity_server::IdentityServerConfigs,
    /// MQTT broker configuration
    pub mqtt: mqtt::MQTTConfigs,
    /// RabbitMQ broker configuration
    pub rabbitmq: rabbitmq::RabbitMQConfigs,
    /// Kafka broker configuration
    pub kafka: kafka::KafkaConfigs,
    /// PostgreSQL database configuration
    pub postgres: postgres::PostgresConfigs,
    /// DynamoDB configuration
    pub dynamo: dynamo::DynamoConfigs,
    /// SQLite database configuration
    pub sqlite: sqlite::SqliteConfigs,
    /// InfluxDB configuration
    pub influx: influx::InfluxConfigs,
    /// AWS services configuration
    pub aws: aws::AwsConfigs,
    /// Health and readiness check configuration
    pub health_readiness: health_readiness::HealthReadinessConfigs,
    /// Application-specific dynamic configuration
    pub dynamic: T,
}

impl<T: DynamicConfigs> Configs<T> {
    /// Creates a new `Configs` instance with environments values.
    ///
    /// This method initializes the configuration from environment variables
    /// for all components, including the application-specific dynamic configuration.
    ///
    /// ## Returns
    ///
    /// A new `Configs` instance with environments values.
    pub fn new() -> Self {
        let mut cfg = Self::default();
        cfg.app = app::AppConfigs::new();
        cfg.otlp = otlp::OTLPConfigs::new();
        cfg.identity = identity_server::IdentityServerConfigs::new();
        cfg.mqtt = mqtt::MQTTConfigs::new();
        cfg.rabbitmq = rabbitmq::RabbitMQConfigs::new();
        cfg.kafka = kafka::KafkaConfigs::new();
        cfg.postgres = postgres::PostgresConfigs::new();
        cfg.dynamo = dynamo::DynamoConfigs::new();
        cfg.sqlite = sqlite::SqliteConfigs::new();
        cfg.influx = influx::InfluxConfigs::new();
        cfg.aws = aws::AwsConfigs::new();
        cfg.health_readiness = health_readiness::HealthReadinessConfigs::new();
        cfg.dynamic = T::default();

        cfg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_app_addr() {
        let cfg = app::AppConfigs::default();
        assert_eq!(cfg.app_addr(), format!("{}:{}", cfg.host, cfg.port))
    }
}
