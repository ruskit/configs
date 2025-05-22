// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for OpenTelemetry (OTLP) observability.
//!
//! This module provides configuration options for connecting to
//! and working with OpenTelemetry exporters for metrics and traces.

use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub enum OTLPExporterType {
    /// OpenTelemetry Protocol (OTLP) exporter.
    Otlp,
    /// Standard output (stdout) exporter.
    #[default]
    Stdout,
}

/// Configuration structure for OpenTelemetry exporters.
///
/// This structure defines the connection parameters and settings for working
/// with OpenTelemetry exporters, including endpoint, authentication, timeouts,
/// and sampling rates.
///
/// ## Examples
///
/// ```
/// use configs::OTLPConfigs;
///
/// let mut otlp_config = OTLPConfigs::default();
/// otlp_config.traces_enabled = true;
/// otlp_config.metrics_enabled = true;
/// ```
#[derive(Debug, Clone)]
pub struct OTLPConfigs {
    /// ENV KEY: "OTLP_EXPORTER_TYPE"
    ///
    /// The type of the OTLP exporter. Possible values are "otlp" or "stdout".
    pub exporter_type: OTLPExporterType,

    /// ENV KEY: "OTLP_EXPORTER_ENDPOINT"
    ///
    /// The endpoint for the OTLP service.
    pub endpoint: String,
    /// ENV KEY: "OTLP_ACCESS_KEY"
    ///
    /// The authentication token for the OTLP service.
    pub access_key: String,
    /// ENV KEY: "OTLP_EXPORTER_TIMEOUT"
    ///
    /// The timeout duration for the OTLP service.
    pub exporter_timeout: Duration,
    /// ENV KEY: "OTLP_EXPORTER_INTERVAL"
    ///
    /// The interval duration for the OTLP service.
    pub exporter_interval: Duration,
    /// ENV KEY: "OTLP_EXPORTER_RATE_BASE"
    ///
    /// The base rate for the OTLP service.
    pub exporter_rate_base: f64,
    /// ENV KEY: "OTLP_METRIC_EXPORTER_RATE_BASE"
    ///
    /// The base rate for the OTLP Metric service.
    pub metric_exporter_rate_base: f64,
    /// ENV KEY: "OTLP_TRACE_EXPORTER_RATE_BASE"
    ///
    /// The base rate for the OTLP Trace service.
    pub trace_exporter_rate_base: f64,
    /// ENV KEY: "OTLP_METRICS_ENABLED"
    ///
    /// The flag to enable or disable metrics.
    pub metrics_enabled: bool,
    /// ENV KEY: "OTLP_TRACES_ENABLED"
    ///
    /// The flag to enable or disable traces.
    pub traces_enabled: bool,
}

pub const OTLP_EXPORTER_TYPE_ENV_KEY: &str = "OTLP_EXPORTER_TYPE";
pub const OTLP_EXPORTER_ENDPOINT_ENV_KEY: &str = "OTLP_EXPORTER_ENDPOINT";
pub const OTLP_ACCESS_KEY_ENV_KEY: &str = "OTLP_ACCESS_KEY";
pub const OTLP_EXPORTER_TIMEOUT_ENV_KEY: &str = "OTLP_EXPORTER_TIMEOUT";
pub const OTLP_EXPORTER_INTERVAL_ENV_KEY: &str = "OTLP_EXPORTER_INTERVAL";
pub const OTLP_EXPORTER_RATE_BASE_ENV_KEY: &str = "OTLP_EXPORTER_RATE_BASE";
pub const OTLP_METRIC_EXPORTER_RATE_BASE_ENV_KEY: &str = "OTLP_METRIC_EXPORTER_RATE_BASE";
pub const OTLP_TRACE_EXPORTER_RATE_BASE_ENV_KEY: &str = "OTLP_TRACE_EXPORTER_RATE_BASE";
pub const OTLP_METRICS_ENABLED_ENV_KEY: &str = "OTLP_METRICS_ENABLED";
pub const OTLP_TRACES_ENABLED_KEY_ENV_KEY: &str = "OTLP_TRACES_ENABLED";

impl OTLPConfigs {
    /// Creates a new instance of `OTLPConfigs` from environment variable
    pub fn new() -> Self {
        let mut cfg = Self::default();

        cfg.exporter_type = std::env::var(OTLP_EXPORTER_TYPE_ENV_KEY)
            .unwrap_or("stdout".to_string())
            .as_str()
            .into();
        cfg.endpoint = std::env::var(OTLP_EXPORTER_ENDPOINT_ENV_KEY).unwrap_or(cfg.endpoint);
        cfg.access_key = std::env::var(OTLP_ACCESS_KEY_ENV_KEY).unwrap_or(cfg.access_key);
        cfg.exporter_timeout = std::env::var(OTLP_EXPORTER_TIMEOUT_ENV_KEY)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(cfg.exporter_timeout);
        cfg.exporter_interval = std::env::var(OTLP_EXPORTER_INTERVAL_ENV_KEY)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(cfg.exporter_interval);
        cfg.exporter_rate_base = std::env::var(OTLP_EXPORTER_RATE_BASE_ENV_KEY)
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(cfg.exporter_rate_base);
        cfg.metric_exporter_rate_base = std::env::var(OTLP_METRIC_EXPORTER_RATE_BASE_ENV_KEY)
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(cfg.metric_exporter_rate_base);
        cfg.trace_exporter_rate_base = std::env::var(OTLP_TRACE_EXPORTER_RATE_BASE_ENV_KEY)
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(cfg.trace_exporter_rate_base);
        cfg.metrics_enabled = std::env::var(OTLP_METRICS_ENABLED_ENV_KEY)
            .ok()
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(cfg.metrics_enabled);
        cfg.traces_enabled = std::env::var(OTLP_TRACES_ENABLED_KEY_ENV_KEY)
            .ok()
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(cfg.traces_enabled);

        return cfg;
    }
}

impl Default for OTLPConfigs {
    fn default() -> Self {
        Self {
            exporter_type: OTLPExporterType::default(),
            endpoint: "http://localhost:4317".to_string(),
            access_key: "token".to_string(),
            exporter_timeout: Duration::from_secs(60),
            exporter_interval: Duration::from_secs(60),
            exporter_rate_base: 0.8,
            metric_exporter_rate_base: 0.8,
            trace_exporter_rate_base: 0.8,
            metrics_enabled: false,
            traces_enabled: false,
        }
    }
}

impl From<&str> for OTLPExporterType {
    fn from(value: &str) -> Self {
        match value.to_uppercase().as_str() {
            "otlp" => OTLPExporterType::Otlp,
            _ => OTLPExporterType::Stdout,
        }
    }
}

impl From<&String> for OTLPExporterType {
    fn from(value: &String) -> Self {
        match value.to_uppercase().as_str() {
            "otlp" => OTLPExporterType::Otlp,
            _ => OTLPExporterType::Stdout,
        }
    }
}
