// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

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
    /// ENV KEY: "INFLUX_HOST"
    ///
    /// The InfluxDB server host URL (Default: "http://localhost")
    pub host: String,
    /// ENV KEY: "INFLUX_PORT"
    ///
    /// The InfluxDB server port (Default: 8086)
    pub port: u64,
    /// ENV KEY: "INFLUX_BUCKET"
    ///
    /// The InfluxDB bucket to use (Default: "default")
    pub bucket: String,
    /// ENV KEY: "INFLUX_TOKEN"
    ///
    /// The authentication token for InfluxDB (Default: "token")
    pub token: String,
}

pub const INFLUX_HOST_ENV_KEY: &str = "INFLUX_HOST";
pub const INFLUX_PORT_ENV_KEY: &str = "INFLUX_PORT";
pub const INFLUX_BUCKET_ENV_KEY: &str = "INFLUX_BUCKET";
pub const INFLUX_TOKEN_ENV_KEY: &str = "INFLUX_TOKEN";

impl InfluxConfigs {
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.host = std::env::var(INFLUX_HOST_ENV_KEY).unwrap_or(cfgs.host);
        cfgs.port = std::env::var(INFLUX_PORT_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(cfgs.port);
        cfgs.bucket = std::env::var(INFLUX_BUCKET_ENV_KEY).unwrap_or(cfgs.bucket);
        cfgs.token = std::env::var(INFLUX_TOKEN_ENV_KEY).unwrap_or(cfgs.token);

        cfgs
    }

    /// Returns the formatted server address (host:port).
    ///
    /// ## Returns
    ///
    /// A String containing the formatted address (e.g., "http://localhost:8086").
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
