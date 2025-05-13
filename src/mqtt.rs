// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for MQTT message brokers.
//!
//! This module provides configuration options for connecting to and
//! working with MQTT message brokers, including different transport
//! protocols and broker types.

use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// # MQTTBrokerKind
///
/// Enum representing supported MQTT broker types.
///
/// This enum defines the different MQTT broker implementations
/// that can be used with the configuration.
///
/// ## Variants
///
/// * `Default` - Standard MQTT broker implementation (default)
/// * `AWSIoTCore` - AWS IoT Core MQTT broker
///
/// ## Examples
///
/// ```
/// use configs::MQTTBrokerKind;
///
/// // Create a broker kind from a string
/// let broker_kind = MQTTBrokerKind::from("AWSIoTCore");
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum MQTTBrokerKind {
    /// Standard MQTT broker (default)
    #[default]
    #[serde(alias = "default")]
    Default,
    /// AWS IoT Core MQTT broker
    #[serde(alias = "awsiotcore")]
    AWSIoTCore,
}

impl From<&str> for MQTTBrokerKind {
    /// Creates a `MQTTBrokerKind` from a string slice.
    ///
    /// The conversion is case-insensitive. If the input string does not match any
    /// known broker kind, it defaults to `MQTTBrokerKind::Default`.
    ///
    /// ## Parameters
    ///
    /// * `value` - A string slice containing the name of the broker kind
    ///
    /// ## Returns
    ///
    /// A `MQTTBrokerKind` variant corresponding to the input string
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "AWSIOTCORE" => MQTTBrokerKind::AWSIoTCore,
            _ => MQTTBrokerKind::Default,
        }
    }
}

impl From<&String> for MQTTBrokerKind {
    /// Creates a `MQTTBrokerKind` from a reference to a String.
    ///
    /// The conversion is case-insensitive. If the input string does not match any
    /// known broker kind, it defaults to `MQTTBrokerKind::Default`.
    ///
    /// ## Parameters
    ///
    /// * `value` - A reference to a String containing the name of the broker kind
    ///
    /// ## Returns
    ///
    /// A `MQTTBrokerKind` variant corresponding to the input string
    fn from(value: &String) -> Self {
        match value.to_uppercase().as_str() {
            "AWSIOTCORE" => MQTTBrokerKind::AWSIoTCore,
            _ => MQTTBrokerKind::Default,
        }
    }
}

/// # MQTTTransport
///
/// Enum representing supported MQTT transport protocols.
///
/// This enum defines the available transport protocols that can be used
/// for MQTT connections.
///
/// ## Variants
///
/// * `TCP` - Standard TCP transport (default)
/// * `SSL` - Secure SSL/TLS transport
/// * `WS` - WebSocket transport
///
/// ## Examples
///
/// ```
/// use configs::MQTTTransport;
///
/// // Create a transport from a string
/// let transport = MQTTTransport::from("SSL");
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum MQTTTransport {
    /// Standard TCP transport (default)
    #[default]
    #[serde(alias = "tcp")]
    TCP,
    /// Secure SSL/TLS transport
    #[serde(alias = "ssl")]
    SSL,
    /// WebSocket transport
    #[serde(alias = "ws")]
    WS,
}

impl From<&str> for MQTTTransport {
    /// Creates an `MQTTTransport` from a string slice.
    ///
    /// The conversion is case-insensitive. If the input string does not match any
    /// known transport, it defaults to `MQTTTransport::TCP`.
    ///
    /// ## Parameters
    ///
    /// * `value` - A string slice containing the name of the transport
    ///
    /// ## Returns
    ///
    /// An `MQTTTransport` variant corresponding to the input string
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "SSL" => MQTTTransport::SSL,
            "WS" => MQTTTransport::WS,
            _ => MQTTTransport::TCP,
        }
    }
}

impl From<&String> for MQTTTransport {
    /// Creates an `MQTTTransport` from a reference to a String.
    ///
    /// The conversion is case-insensitive. If the input string does not match any
    /// known transport, it defaults to `MQTTTransport::TCP`.
    ///
    /// ## Parameters
    ///
    /// * `value` - A reference to a String containing the name of the transport
    ///
    /// ## Returns
    ///
    /// An `MQTTTransport` variant corresponding to the input string
    fn from(value: &String) -> Self {
        match value.to_uppercase().as_str() {
            "SSL" => MQTTTransport::SSL,
            "WS" => MQTTTransport::WS,
            _ => MQTTTransport::TCP,
        }
    }
}

impl Display for MQTTTransport {
    /// Formats the `MQTTTransport` for display.
    ///
    /// ## Returns
    ///
    /// A string representation of the transport (e.g., "tcp", "ssl", "ws").
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MQTTTransport::TCP => write!(f, "tcp"),
            MQTTTransport::SSL => write!(f, "ssl"),
            MQTTTransport::WS => write!(f, "ws"),
        }
    }
}

/// # MQTTConnectionConfigs
///
/// Configuration structure for a single MQTT broker connection.
///
/// This structure defines the connection parameters for an MQTT broker,
/// including host, port, authentication, and transport settings.
///
/// ## Examples
///
/// ```
/// use configs::{MQTTConnectionConfigs, MQTTTransport};
///
/// let mut mqtt_connection = MQTTConnectionConfigs::default();
/// mqtt_connection.host = "mqtt.example.com".to_string();
/// mqtt_connection.transport = MQTTTransport::SSL;
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MQTTConnectionConfigs {
    /// ENV KEY: "TAG"
    ///
    /// A unique identifier for this connection configuration (Default: "default")
    pub tag: String,

    /// ENV KEY: "MQTT_BROKER_KIND"
    ///
    /// The type of MQTT broker (Default: MQTTBrokerKind::Default)
    #[serde(default)]
    pub broker_kind: MQTTBrokerKind,

    /// ENV KEY: "MQTT_HOST"
    ///
    /// The MQTT broker host (Default: "localhost")
    pub host: String,

    /// ENV KEY: "MQTT_TRANSPORT"
    ///
    /// The transport protocol to use (Default: MQTTTransport::TCP)
    pub transport: MQTTTransport,

    /// ENV KEY: "MQTT_PORT"
    ///
    /// The MQTT broker port (Default: 1883)
    pub port: u64,

    /// ENV KEY: "MQTT_USER"
    ///
    /// Username for MQTT authentication (Default: "mqtt_user")
    pub user: String,

    /// ENV KEY: "MQTT_PASSWORD"
    ///
    /// Password for MQTT authentication (Default: "password")
    pub password: String,

    /// Device name for cloud MQTT brokers (Default: "")
    ///
    /// Used with Public Cloud Brokers
    #[serde(default)]
    pub device_name: String,

    /// ENV KEY: "MQTT_CA_CERT_PATH"
    ///
    /// Path to the root CA certificate file (Default: "")
    ///
    /// Used with Public Cloud Brokers
    #[serde(default)]
    pub root_ca_path: String,

    /// ENV KEY: "MQTT_CERT_PATH"
    ///
    /// Path to the client certificate file (Default: "")
    ///
    /// Used with Public Cloud Brokers
    #[serde(default)]
    pub cert_path: String,

    /// ENV KEY: "MQTT_PRIVATE_KEY_PATH"
    ///
    /// Path to the private key file (Default: "")
    ///
    /// Used with Public Cloud Brokers
    #[serde(default)]
    pub private_key_path: String,
}

/// # MQTTConfigs
///
/// Configuration structure for MQTT connections.
///
/// This structure supports both single and multiple MQTT broker configurations,
/// allowing connections to different MQTT brokers with different settings.
///
/// ## Examples
///
/// ```
/// use configs::{MQTTConfigs, MQTTConnectionConfigs};
///
/// let mut mqtt_config = MQTTConfigs::default();
///
/// // Enable multi-broker mode
/// mqtt_config.multi_broker_enabled = true;
///
/// // Add another broker connection
/// let second_broker = MQTTConnectionConfigs::default();
/// mqtt_config.connection_configs.push(second_broker);
/// ```
#[derive(Debug, Clone)]
pub struct MQTTConfigs {
    /// ENV KEY: "MQTT_MULTI_BROKER_ENABLED"
    ///
    /// Whether multi-broker mode is enabled (Default: false)
    pub multi_broker_enabled: bool,
    /// ENV KEY: "MQTT_BROKERS"
    ///
    /// JSON string containing a list of MQTT brokers (Default: "[]")
    pub brokers: String,
    ///
    /// List of MQTT connection configurations
    pub connection_configs: Vec<MQTTConnectionConfigs>,
}

pub const MQTT_MULTI_BROKER_ENABLED_ENV_KEY: &str = "MQTT_MULTI_BROKER_ENABLED";
pub const MQTT_BROKERS_ENV_KEY: &str = "MQTT_BROKERS";
pub const MQTT_BROKER_KIND_ENV_KEY: &str = "MQTT_BROKER_KIND";
pub const MQTT_HOST_ENV_KEY: &str = "MQTT_HOST";
pub const MQTT_TRANSPORT_ENV_KEY: &str = "MQTT_TRANSPORT";
pub const MQTT_PORT_ENV_KEY: &str = "MQTT_PORT";
pub const MQTT_USER_ENV_KEY: &str = "MQTT_USER";
pub const MQTT_PASSWORD_ENV_KEY: &str = "MQTT_PASSWORD";
pub const MQTT_CA_CERT_PATH_ENV_KEY: &str = "MQTT_CA_CERT_PATH";
pub const MQTT_CERT_PATH_ENV_KEY: &str = "MQTT_CERT_PATH";
pub const MQTT_PRIVATE_KEY_PATH_ENV_KEY: &str = "MQTT_PRIVATE_KEY_PATH";

impl MQTTConfigs {
    /// Creates a new `MQTTConfigs` with environment variables.
    ///
    /// This method initializes the MQTT configuration with environment variables
    /// for multi-broker mode and the list of brokers.
    ///
    /// ## Returns
    ///
    /// A new `MQTTConfigs` with environment variables.
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.multi_broker_enabled = std::env::var(MQTT_MULTI_BROKER_ENABLED_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<bool>().ok())
            .unwrap_or(cfgs.multi_broker_enabled);

        if !cfgs.multi_broker_enabled {
            let mut conn_configs = MQTTConnectionConfigs::default();

            conn_configs.tag = "default".into();
            conn_configs.broker_kind = std::env::var(MQTT_BROKER_KIND_ENV_KEY)
                .ok()
                .map(|v| MQTTBrokerKind::from(&v))
                .unwrap_or(conn_configs.broker_kind);
            conn_configs.host = std::env::var(MQTT_HOST_ENV_KEY).unwrap_or(conn_configs.host);
            conn_configs.transport = std::env::var(MQTT_TRANSPORT_ENV_KEY)
                .ok()
                .map(|v| MQTTTransport::from(&v))
                .unwrap_or(conn_configs.transport);
            conn_configs.port = std::env::var(MQTT_PORT_ENV_KEY)
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(conn_configs.port);
            conn_configs.user = std::env::var(MQTT_USER_ENV_KEY).unwrap_or(conn_configs.user);
            conn_configs.password =
                std::env::var(MQTT_PASSWORD_ENV_KEY).unwrap_or(conn_configs.password);
            conn_configs.root_ca_path =
                std::env::var(MQTT_CA_CERT_PATH_ENV_KEY).unwrap_or(conn_configs.root_ca_path);
            conn_configs.cert_path =
                std::env::var(MQTT_CERT_PATH_ENV_KEY).unwrap_or(conn_configs.cert_path);
            conn_configs.private_key_path = std::env::var(MQTT_PRIVATE_KEY_PATH_ENV_KEY)
                .unwrap_or(conn_configs.private_key_path);
        }

        cfgs.brokers = std::env::var(MQTT_BROKERS_ENV_KEY).unwrap_or(cfgs.brokers);

        cfgs
    }
}

impl Default for MQTTConfigs {
    fn default() -> Self {
        let mut cfg = MQTTConnectionConfigs::default();
        cfg.tag = "default".into();

        Self {
            multi_broker_enabled: false,
            brokers: "[]".into(),
            connection_configs: vec![cfg],
        }
    }
}
