# Configs

[![Crate](https://img.shields.io/crates/v/configs.svg)](https://crates.io/crates/configs)
[![Docs](https://docs.rs/configs/badge.svg)](https://docs.rs/configs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A flexible configuration management library for Rust applications in the Ruskit ecosystem.

## Overview

The `configs` crate provides a comprehensive set of configuration structures for various services and components commonly used in Rust applications. It enables seamless configuration of:

- AWS services
- Database connections (PostgreSQL, SQLite, DynamoDB)
- Message brokers (RabbitMQ, Kafka, MQTT)
- Observability tooling (metrics, tracing)
- Security integrations (identity servers, secrets management)
- Application health checks

## Features

- Environment-aware configuration (Local, Dev, Staging, Production)
- Extensible design through the `DynamicConfigs` trait
- Default configurations for quick prototyping
- Consistent interface across all service configurations
- Helper methods for common configuration tasks

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
configs = { version = "0.1.0" }
```

### Basic Example

```rust
use configs::{Configs, Empty, Environment};

fn main() {
    // Create a new configuration with default values
    let config = Configs::<Empty>::default();
    
    // Access specific configuration components
    let app_addr = config.app.app_addr();
    println!("App will listen on {}", app_addr);
    
    // Generate a connection URI for RabbitMQ
    let rabbitmq_uri = config.rabbitmq_uri();
    println!("RabbitMQ URI: {}", rabbitmq_uri);
}
```

### Custom Application Configuration

You can extend the configuration with your own application-specific settings by implementing the `DynamicConfigs` trait:

```rust
use configs::{Configs, DynamicConfigs};
use std::env;

#[derive(Debug, Clone, Default)]
struct MyAppConfig {
    feature_flag_enabled: bool,
    max_connections: u32,
}

impl DynamicConfigs for MyAppConfig {
    fn load(&mut self) {
        // Load from environment variables or other sources
        self.feature_flag_enabled = env::var("FEATURE_FLAG")
            .unwrap_or_default()
            .to_lowercase() == "true";
            
        self.max_connections = env::var("MAX_CONNECTIONS")
            .unwrap_or_default()
            .parse()
            .unwrap_or(100);
    }
}

fn main() {
    let mut config = Configs::<MyAppConfig>::default();
    
    // Load application-specific configuration
    config.dynamic.load();
    
    if config.dynamic.feature_flag_enabled {
        println!("Feature flag is enabled!");
    }
}
```

## Documentation

For detailed documentation and examples, see the [API documentation](https://docs.rs/configs).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.