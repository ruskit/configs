//! Defines configuration structures for distributed tracing.
//!
//! This module provides configuration options for collecting and exporting
//! distributed traces using OpenTelemetry.

use std::str::FromStr;

/// # TraceExporterKind
///
/// Enum representing supported trace export formats and protocols.
///
/// This enum defines the available trace exporters that can be used to
/// send application traces to external observability systems.
///
/// ## Variants
///
/// * `Stdout` - Write traces to standard output (default)
/// * `OtlpGrpc` - Export traces using OpenTelemetry Protocol over gRPC
///
/// ## Examples
///
/// ```
/// use std::str::FromStr;
/// use configs::TraceExporterKind;
///
/// // Parse from configuration string
/// let exporter = TraceExporterKind::from_str("otlp").unwrap();
/// ```
#[derive(Debug, Clone, Default)]
pub enum TraceExporterKind {
    /// Output traces to stdout (default)
    #[default]
    Stdout,
    /// Export traces using OpenTelemetry Protocol over gRPC
    OtlpGrpc,
}

impl FromStr for TraceExporterKind {
    type Err = String;

    /// Creates a `TraceExporterKind` from a string slice.
    ///
    /// The conversion is case-insensitive and accepts multiple aliases for each exporter.
    ///
    /// ## Parameters
    ///
    /// * `s` - A string slice containing the name of the exporter
    ///
    /// ## Returns
    ///
    /// A `Result` containing the corresponding `TraceExporterKind` or an error message
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "otlp" | "otlp-grpc" | "grpc" => Ok(TraceExporterKind::OtlpGrpc),
            _ => Ok(TraceExporterKind::Stdout),
        }
    }
}

/// # TraceConfigs
///
/// Configuration structure for distributed tracing.
///
/// This structure defines the parameters for collecting and exporting distributed
/// traces, including which exporter to use and configuration specific to each
/// exporter type.
///
/// ## Examples
///
/// ```
/// use configs::{TraceConfigs, TraceExporterKind};
///
/// let mut trace_config = TraceConfigs::default();
/// trace_config.enable = true;
/// trace_config.exporter = TraceExporterKind::OtlpGrpc;
/// trace_config.host = "otel-collector.example.com".to_string();
/// ```
#[derive(Debug, Clone)]
pub struct TraceConfigs {
    /// Whether distributed tracing is enabled (Default: false)
    pub enable: bool,
    /// The traces exporter to use (Default: TraceExporterKind::Stdout)
    pub exporter: TraceExporterKind,
    /// The host address for the OTLP exporter (Default: "")
    pub host: String,
    /// Header name for access key authentication (Default: "")
    pub header_access_key: String,
    /// Access key value for authentication (Default: "")
    pub access_key: String,
    /// Service type identifier for traces (Default: "")
    pub service_type: String,
    /// Timeout for trace export operations in seconds (Default: 30)
    pub export_timeout: u64,
    /// Interval between trace exports in seconds (Default: 60)
    pub export_interval: u64,
    /// Base rate for export sampling (Default: 0.8)
    pub export_rate_base: f64,
}

impl Default for TraceConfigs {
    fn default() -> Self {
        Self {
            enable: false,
            host: Default::default(),
            exporter: Default::default(),
            header_access_key: Default::default(),
            access_key: Default::default(),
            service_type: Default::default(),
            export_timeout: 30,
            export_interval: 60,
            export_rate_base: 0.8,
        }
    }
}
