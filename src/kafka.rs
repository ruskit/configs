//! Defines configuration structures for Apache Kafka.
//!
//! This module provides configuration options for connecting to and
//! working with Apache Kafka message brokers.

/// # KafkaConfigs
///
/// Configuration structure for Apache Kafka connections.
///
/// This structure defines the connection parameters and security settings for working
/// with Apache Kafka message brokers, including host, port, authentication credentials,
/// and SSL/TLS configuration options.
///
/// ## Examples
///
/// ```
/// use configs::KafkaConfigs;
///
/// let kafka_config = KafkaConfigs::default();
/// // Use Kafka configuration with Kafka client
/// ```
#[derive(Debug, Clone)]
pub struct KafkaConfigs {
    /// The Kafka broker host (Default: "localhost")
    pub host: String,
    /// The Kafka broker port (Default: 9094)
    pub port: u64,
    /// Connection timeout in milliseconds (Default: 6000)
    pub timeout: u64,
    /// Security protocol for Kafka connections (Default: "SASL_SSL")
    pub security_protocol: String,
    /// SASL mechanism for authentication (Default: "PLAIN")
    pub sasl_mechanisms: String,
    /// Path to the SSL certificate file (Default: "")
    pub certificate_path: String,
    /// Path to the CA certificate file (Default: "")
    pub ca_path: String,
    /// Path to the trust store (Default: "")
    pub trust_store_path: String,
    /// Password for the trust store (Default: "")
    pub trust_store_password: String,
    /// Path to the key store (Default: "")
    pub key_store_path: String,
    /// Password for the key store (Default: "")
    pub key_store_password: String,
    /// Algorithm for endpoint identification (Default: "")
    pub endpoint_identification_algorithm: String,
    /// SASL username (Default: "")
    pub user: String,
    /// SASL password (Default: "")
    pub password: String,
}

impl Default for KafkaConfigs {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 9094,
            timeout: 6000,
            security_protocol: "SASL_SSL".into(),
            sasl_mechanisms: "PLAIN".into(),
            certificate_path: String::default(),
            ca_path: String::default(),
            trust_store_path: Default::default(),
            trust_store_password: Default::default(),
            key_store_path: Default::default(),
            key_store_password: Default::default(),
            endpoint_identification_algorithm: Default::default(),
            user: Default::default(),
            password: Default::default(),
        }
    }
}
