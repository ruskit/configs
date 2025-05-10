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
    /// A unique identifier for this connection configuration (Default: "default")
    pub tag: String,

    /// The type of MQTT broker (Default: MQTTBrokerKind::Default)
    #[serde(default)]
    pub broker_kind: MQTTBrokerKind,

    /// The MQTT broker host (Default: "localhost")
    pub host: String,

    /// The transport protocol to use (Default: MQTTTransport::TCP)
    pub transport: MQTTTransport,

    /// The MQTT broker port (Default: 1883)
    pub port: u64,

    /// Username for MQTT authentication (Default: "mqtt_user")
    pub user: String,

    /// Password for MQTT authentication (Default: "password")
    pub password: String,

    /// Device name for cloud MQTT brokers (Default: "")
    ///
    /// Used with Public Cloud Brokers
    #[serde(default)]
    pub device_name: String,

    /// Path to the root CA certificate file (Default: "")
    ///
    /// Used with Public Cloud Brokers
    #[serde(default)]
    pub root_ca_path: String,

    /// Path to the client certificate file (Default: "")
    ///
    /// Used with Public Cloud Brokers
    #[serde(default)]
    pub cert_path: String,

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
    /// Whether multi-broker mode is enabled (Default: false)
    pub multi_broker_enabled: bool,
    /// List of MQTT connection configurations
    pub connection_configs: Vec<MQTTConnectionConfigs>,
}

impl Default for MQTTConfigs {
    fn default() -> Self {
        let mut cfg = MQTTConnectionConfigs::default();
        cfg.tag = "default".into();

        Self {
            multi_broker_enabled: false,
            connection_configs: vec![cfg],
        }
    }
}
