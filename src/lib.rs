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

pub mod app;
pub mod aws;
pub mod configs;
pub mod dynamic;
pub mod dynamo;
pub mod environment;
pub mod health_readiness;
pub mod identity_server;
pub mod influx;
pub mod kafka;
pub mod mqtt;
pub mod otlp;
pub mod postgres;
pub mod rabbitmq;
pub mod secrets;
pub mod sqlite;
