// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

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
    /// ENV KEY: "KAFKA_HOST"
    ///
    /// The Kafka broker host (Default: "localhost")
    pub host: String,
    /// ENV KEY: "KAFKA_PORT"
    ///
    /// The Kafka broker port (Default: 9094)
    pub port: u64,
    /// ENV KEY: "KAFKA_TIMEOUT"
    ///
    /// Connection timeout in milliseconds (Default: 6000)
    pub timeout: u64,
    /// ENV KEY: "KAFKA_SECURITY_PROTOCOL"
    ///
    /// Security protocol for Kafka connections (Default: "SASL_SSL")
    pub security_protocol: String,
    /// ENV KEY: "KAFKA_SASL_MECHANISMS"
    ///
    /// SASL mechanism for authentication (Default: "PLAIN")
    pub sasl_mechanisms: String,
    /// ENV KEY: "KAFKA_CERTIFICATE_PATH"
    ///
    /// Path to the SSL certificate file (Default: "")
    pub certificate_path: String,
    /// ENV KEY: "KAFKA_CA_PATH"
    ///
    /// Path to the CA certificate file (Default: "")
    pub ca_path: String,
    /// ENV KEY: "KAFKA_TRUST_STORE_PATH"
    ///
    /// Path to the trust store (Default: "")
    pub trust_store_path: String,
    /// ENV KEY: "KAFKA_TRUST_STORE_PASSWORD"
    ///
    /// Password for the trust store (Default: "")
    pub trust_store_password: String,
    /// ENV KEY: "KAFKA_KEY_STORE_PATH"
    ///
    /// Path to the key store (Default: "")
    pub key_store_path: String,
    /// ENV KEY: "KAFKA_KEY_STORE_PASSWORD"
    ///
    /// Password for the key store (Default: "")
    pub key_store_password: String,
    /// ENV KEY: "KAFKA_ENDPOINT_IDENTIFICATION_ALGORITHM"
    ///
    /// Algorithm for endpoint identification (Default: "")
    pub endpoint_identification_algorithm: String,
    /// ENV KEY: "KAFKA_USER"
    ///
    /// SASL username (Default: "")
    pub user: String,
    /// ENV KEY: "KAFKA_PASSWORD"
    ///
    /// SASL password (Default: "")
    pub password: String,
}

pub const KAFKA_HOST_ENV_KEY: &str = "KAFKA_HOST";
pub const KAFKA_PORT_ENV_KEY: &str = "KAFKA_PORT";
pub const KAFKA_TIMEOUT_ENV_KEY: &str = "KAFKA_TIMEOUT";
pub const KAFKA_SECURITY_PROTOCOL_ENV_KEY: &str = "KAFKA_SECURITY_PROTOCOL";
pub const KAFKA_SASL_MECHANISMS_ENV_KEY: &str = "KAFKA_SASL_MECHANISMS";
pub const KAFKA_CERTIFICATE_PATH_KEY: &str = "KAFKA_CERTIFICATE_PATH";
pub const KAFKA_CA_PATH_KEY: &str = "KAFKA_CA_PATH";
pub const KAFKA_TRUST_STORE_PATH_KEY: &str = "KAFKA_TRUST_STORE_PATH";
pub const KAFKA_TRUST_STORE_PASSWORD_KEY: &str = "KAFKA_TRUST_STORE_PASSWORD";
pub const KAFKA_KEY_STORE_PATH_KEY: &str = "KAFKA_KEY_STORE_PATH";
pub const KAFKA_KEY_STORE_PASSWORD_KEY: &str = "KAFKA_KEY_STORE_PASSWORD";
pub const KAFKA_ENDPOINT_IDENTIFICATION_ALGORITHM_KEY: &str =
    "KAFKA_ENDPOINT_IDENTIFICATION_ALGORITHM";
pub const KAFKA_USER_ENV_KEY: &str = "KAFKA_USER";
pub const KAFKA_PASSWORD_ENV_KEY: &str = "KAFKA_PASSWORD";

impl KafkaConfigs {
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.host = std::env::var(KAFKA_HOST_ENV_KEY).unwrap_or(cfgs.host);
        cfgs.port = std::env::var(KAFKA_PORT_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(cfgs.port);
        cfgs.timeout = std::env::var(KAFKA_TIMEOUT_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(cfgs.timeout);
        cfgs.security_protocol =
            std::env::var(KAFKA_SECURITY_PROTOCOL_ENV_KEY).unwrap_or(cfgs.security_protocol);
        cfgs.sasl_mechanisms =
            std::env::var(KAFKA_SASL_MECHANISMS_ENV_KEY).unwrap_or(cfgs.sasl_mechanisms);
        cfgs.certificate_path =
            std::env::var(KAFKA_CERTIFICATE_PATH_KEY).unwrap_or(cfgs.certificate_path);
        cfgs.ca_path = std::env::var(KAFKA_CA_PATH_KEY).unwrap_or(cfgs.ca_path);
        cfgs.trust_store_path =
            std::env::var(KAFKA_TRUST_STORE_PATH_KEY).unwrap_or(cfgs.trust_store_path);
        cfgs.trust_store_password =
            std::env::var(KAFKA_TRUST_STORE_PASSWORD_KEY).unwrap_or(cfgs.trust_store_password);
        cfgs.key_store_path =
            std::env::var(KAFKA_KEY_STORE_PATH_KEY).unwrap_or(cfgs.key_store_path);
        cfgs.key_store_password =
            std::env::var(KAFKA_KEY_STORE_PASSWORD_KEY).unwrap_or(cfgs.key_store_password);
        cfgs.endpoint_identification_algorithm =
            std::env::var(KAFKA_ENDPOINT_IDENTIFICATION_ALGORITHM_KEY)
                .unwrap_or(cfgs.endpoint_identification_algorithm);
        cfgs.user = std::env::var(KAFKA_USER_ENV_KEY).unwrap_or(cfgs.user);
        cfgs.password = std::env::var(KAFKA_PASSWORD_ENV_KEY).unwrap_or(cfgs.password);

        cfgs
    }
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
