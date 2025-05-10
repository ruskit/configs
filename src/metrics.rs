//! Defines configuration structures for application metrics.
//!
//! This module provides configuration options for metrics collection
//! and export to various metrics backends (Prometheus, OTLP, etc.)

use std::str::FromStr;

/// # MetricExporterKind
///
/// Enum representing supported metrics export formats and protocols.
///
/// This enum defines the available metrics exporters that can be used to
/// send application metrics to external monitoring systems.
///
/// ## Variants
///
/// * `Stdout` - Write metrics to standard output (default)
/// * `OtlpGrpc` - Export metrics using OpenTelemetry Protocol over gRPC
/// * `Prometheus` - Expose metrics in Prometheus format via HTTP endpoint
///
/// ## Examples
///
/// ```
/// use std::str::FromStr;
/// use configs::MetricExporterKind;
///
/// // Parse from configuration string
/// let exporter = MetricExporterKind::from_str("prometheus").unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub enum MetricExporterKind {
    /// Output metrics to stdout (default)
    #[default]
    Stdout,
    /// Export metrics using OpenTelemetry Protocol over gRPC
    OtlpGrpc,
    /// Expose metrics in Prometheus format via HTTP endpoint
    Prometheus,
}

impl FromStr for MetricExporterKind {
    type Err = String;

    /// Creates a `MetricExporterKind` from a string slice.
    ///
    /// The conversion is case-insensitive and accepts multiple aliases for each exporter.
    ///
    /// ## Parameters
    ///
    /// * `s` - A string slice containing the name of the exporter
    ///
    /// ## Returns
    ///
    /// A `Result` containing the corresponding `MetricExporterKind` or an error message
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "otlp" | "otlp-grpc" | "grpc" => Ok(MetricExporterKind::OtlpGrpc),
            "prom" | "prometheus" => Ok(MetricExporterKind::Prometheus),
            _ => Ok(MetricExporterKind::Stdout),
        }
    }
}

/// # MetricConfigs
///
/// Configuration structure for application metrics collection and export.
///
/// This structure defines the parameters for collecting and exporting application
/// metrics, including which exporter to use and configuration specific to each
/// exporter type.
///
/// ## Examples
///
/// ```
/// use configs::{MetricConfigs, MetricExporterKind};
///
/// let mut metric_config = MetricConfigs::default();
/// metric_config.enable = true;
/// metric_config.exporter = MetricExporterKind::Prometheus;
/// ```
#[derive(Debug, Clone)]
pub struct MetricConfigs {
    /// Whether metrics collection is enabled (Default: false)
    pub enable: bool,
    /// The metrics exporter to use (Default: MetricExporterKind::Stdout)
    pub exporter: MetricExporterKind,
    /// The host address for the OTLP exporter (Default: "")
    ///
    /// Only used with OTLP exporter
    pub host: String,
    /// Header name for access key authentication (Default: "")
    ///
    /// Only used with OTLP exporter
    pub header_access_key: String,
    /// Access key value for authentication (Default: "")
    ///
    /// Only used with OTLP exporter
    pub access_key: String,
    /// Service type identifier for metrics (Default: "")
    pub service_type: String,
    /// Timeout for metric export operations in seconds (Default: 30)
    ///
    /// Only used with OTLP exporter
    pub export_timeout: u64,
    /// Interval between metric exports in seconds (Default: 60)
    ///
    /// Only used with OTLP exporter
    pub export_interval: u64,
    /// Base rate for export sampling (Default: 0.8)
    ///
    /// Only used with OTLP exporter
    pub export_rate_base: f64,
}

impl Default for MetricConfigs {
    fn default() -> Self {
        Self {
            enable: false,
            exporter: Default::default(),
            host: Default::default(),
            header_access_key: Default::default(),
            access_key: Default::default(),
            service_type: Default::default(),
            export_timeout: 30,
            export_interval: 60,
            export_rate_base: 0.8,
        }
    }
}
