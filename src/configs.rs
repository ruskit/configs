//! Provides the central configuration aggregation structures and traits.
//!
//! This module defines the main `Configs` structure that aggregates all service-specific 
//! configurations, as well as the `DynamicConfigs` trait for extensibility.

use crate::{
    kafka::KafkaConfigs, AppConfigs, AwsConfigs, DynamoConfigs, HealthReadinessConfigs, IdentityServerConfigs, InfluxConfigs, MQTTConfigs, MetricConfigs, PostgresConfigs, RabbitMQConfigs, SqliteConfigs, TraceConfigs
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
    pub app: AppConfigs,
    /// Identity server configuration
    pub identity: IdentityServerConfigs,
    /// MQTT broker configuration
    pub mqtt: MQTTConfigs,
    /// RabbitMQ broker configuration
    pub rabbitmq: RabbitMQConfigs,
    /// Kafka broker configuration
    pub kafka: KafkaConfigs,
    /// Metrics configuration
    pub metric: MetricConfigs,
    /// Distributed tracing configuration
    pub trace: TraceConfigs,
    /// PostgreSQL database configuration
    pub postgres: PostgresConfigs,
    /// DynamoDB configuration
    pub dynamo: DynamoConfigs,
    /// SQLite database configuration
    pub sqlite: SqliteConfigs,
    /// InfluxDB configuration
    pub influx: InfluxConfigs,
    /// AWS services configuration
    pub aws: AwsConfigs,
    /// Health and readiness check configuration
    pub health_readiness: HealthReadinessConfigs,
    /// Application-specific dynamic configuration
    pub dynamic: T,
}

/// # DynamicConfigs
///
/// A trait for application-specific configuration extensions.
///
/// Implementors of this trait can provide custom configuration loading logic
/// for application-specific settings that aren't covered by the standard
/// configuration objects.
///
/// ## Required Methods
///
/// * `load` - Load configuration values from environment variables, files, etc.
pub trait DynamicConfigs: Default {
    /// Load configuration values from the environment or other sources.
    fn load(&mut self);
}

/// # Empty
///
/// A placeholder implementation of `DynamicConfigs` that doesn't add any configuration.
///
/// This type can be used when no additional configuration beyond the standard
/// service configurations is needed.
#[derive(Debug, Default, Clone, Copy)]
pub struct Empty;

impl DynamicConfigs for Empty {
    fn load(&mut self) {}
}

impl<T> Configs<T>
where
    T: DynamicConfigs,
{
    /// Generates a RabbitMQ connection URI from the RabbitMQ configuration.
    ///
    /// ## Returns
    ///
    /// A formatted connection string in the format `amqp://user:password@host:port/vhost`.
    pub fn rabbitmq_uri(&self) -> String {
        format!(
            "amqp://{}:{}@{}:{}{}",
            self.rabbitmq.user,
            self.rabbitmq.password,
            self.rabbitmq.host,
            self.rabbitmq.port,
            self.rabbitmq.vhost
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_app_addr() {
        let cfg = AppConfigs::default();

        assert_eq!(cfg.app_addr(), format!("{}:{}", cfg.host, cfg.port))
    }

    #[test]
    fn should_return_amqp_uri() {
        let cfg = Configs::<Empty>::default();

        assert_eq!(
            cfg.rabbitmq_uri(),
            format!(
                "amqp://{}:{}@{}:{}{}",
                cfg.rabbitmq.user,
                cfg.rabbitmq.password,
                cfg.rabbitmq.host,
                cfg.rabbitmq.port,
                cfg.rabbitmq.vhost
            )
        )
    }
}
