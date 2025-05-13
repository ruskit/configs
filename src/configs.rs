// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Provides the central configuration aggregation structures and traits.
//!
//! This module defines the main `Configs` structure that aggregates all service-specific
//! configurations, as well as the `DynamicConfigs` trait for extensibility.

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
/// // Use the configuration
/// let rabbitmq_uri = config.rabbitmq_uri();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_app_addr() {
        let cfg = app::AppConfigs::default();
        assert_eq!(cfg.app_addr(), format!("{}:{}", cfg.host, cfg.port))
    }
}
