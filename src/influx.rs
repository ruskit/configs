//! Defines configuration structures for InfluxDB.
//!
//! This module provides configuration options for connecting to and
//! working with InfluxDB time-series database.

/// # InfluxConfigs
///
/// Configuration structure for InfluxDB connections.
///
/// This structure defines the connection parameters and settings for working
/// with InfluxDB, including server address, port, bucket, and authentication token.
///
/// ## Examples
///
/// ```
/// use configs::InfluxConfigs;
///
/// let influx_config = InfluxConfigs::default();
/// println!("InfluxDB server address: {}", influx_config.addr());
/// ```
#[derive(Debug, Clone, Default)]
pub struct InfluxConfigs {
    /// The InfluxDB server host URL (Default: "http://localhost")
    pub host: String,
    /// The InfluxDB server port (Default: 8086)
    pub port: u64,
    /// The InfluxDB bucket to use (Default: "default")
    pub bucket: String,
    /// The authentication token for InfluxDB (Default: "token")
    pub token: String,
}

impl InfluxConfigs {
    /// Returns the formatted server address (host:port).
    ///
    /// ## Returns
    ///
    /// A String containing the formatted address (e.g., "http://localhost:8086").
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
