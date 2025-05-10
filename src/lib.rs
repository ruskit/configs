// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Configs
//!
//! A configuration management crate for Rust applications in the Ruskit ecosystem.
//!
//! This crate provides a collection of configuration structures for various services and components,
//! making it easy to configure and manage application settings across different environments.
//!
//! ## Features
//!
//! - Environment-aware configuration (Local, Dev, Staging, Production)
//! - Support for various services (AWS, PostgreSQL, Kafka, RabbitMQ, etc.)
//! - Extensible design through the `DynamicConfigs` trait
//! - Helper methods for common configuration tasks

mod app;
mod aws;
mod configs;
mod dynamo;
mod environment;
mod health_readiness;
mod identity_server;
mod influx;
mod kafka;
mod metrics;
mod mqtt;
mod postgres;
mod rabbitmq;
mod secrets;
mod sqlite;
mod traces;

pub use app::AppConfigs;
pub use aws::AwsConfigs;
pub use configs::{Configs, DynamicConfigs, Empty};
pub use dynamo::DynamoConfigs;
pub use environment::Environment;
pub use health_readiness::HealthReadinessConfigs;
pub use identity_server::IdentityServerConfigs;
pub use influx::InfluxConfigs;
pub use kafka::KafkaConfigs;
pub use metrics::{MetricConfigs, MetricExporterKind};
pub use mqtt::{MQTTBrokerKind, MQTTConfigs, MQTTConnectionConfigs, MQTTTransport};
pub use postgres::{PostgresConfigs, PostgresSslMode};
pub use rabbitmq::RabbitMQConfigs;
pub use secrets::SecretsManagerKind;
pub use sqlite::SqliteConfigs;
pub use traces::{TraceConfigs, TraceExporterKind};
