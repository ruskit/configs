// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Configs
//!
//! A comprehensive configuration management library for Rust applications in the Ruskit ecosystem.
//!
//! This crate provides a centralized way to manage configuration for various services and components
//! commonly used in Rust applications. It handles environment variables, default values, and type
//! conversions, making configuration management simple and consistent.
//!
//! ## Features
//!
//! - **Environment-aware**: Adapts configuration based on deployment environments (Local, Dev, Staging, Production)
//! - **Environment variable integration**: Automatically loads values from environment variables
//! - **Sensible defaults**: Comes with production-ready default configurations
//! - **Type safety**: Strong Rust types for all configuration parameters
//! - **Extensibility**: Customize with application-specific configurations via the `DynamicConfigs` trait
//!
//! ## Usage Examples
//!
//! ```rust
//! use configs::{Configs, Empty};
//!
//! // Create configuration with default values
//! let config = Configs::<Empty>::default();
//!
//! // Access configuration components
//! println!("App will listen on {}", config.app.app_addr());
//! println!("Environment: {}", config.app.env);
//!
//! // Check environment-specific behavior
//! if config.app.env.is_prod() {
//!     println!("Running in production mode");
//! }
//! ```
//!
//! ## Module Structure
//!
//! Each module in this crate provides configuration for a specific service or component:
//!
//! - `app`: Core application configuration (environment, host, port)
//! - `aws`: AWS services configuration
//! - `postgres`, `sqlite`, `dynamo`, `influx`: Database configurations
//! - `rabbitmq`, `kafka`, `mqtt`: Message broker configurations
//! - `otlp`: OpenTelemetry observability configuration
//! - `identity_server`: Authentication and identity provider configuration
//! - And more specialized configuration modules

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
