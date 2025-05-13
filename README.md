# Configs

[![Crate](https://img.shields.io/crates/v/configs.svg)](https://crates.io/crates/configs)
[![Docs](https://docs.rs/configs/badge.svg)](https://docs.rs/configs)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)

A comprehensive configuration management library for Rust applications in the Ruskit ecosystem.

## Overview

The `configs` crate provides a centralized way to manage configuration for various services and components commonly used in Rust applications. It handles environment variables, default values, and type conversions, making configuration management simple and consistent.

This library supports configuration for:

- **Application settings**: Environment, ports, secrets, logging levels
- **Database connections**: PostgreSQL, SQLite, DynamoDB, InfluxDB
- **Message brokers**: RabbitMQ, Kafka, MQTT
- **Cloud services**: AWS (including IoT Core, Secrets Manager)
- **Identity & auth**: OAuth/OpenID Connect providers
- **Observability**: OpenTelemetry (OTLP) for metrics and tracing
- **Health checks**: Kubernetes-compatible health and readiness probes

## Features

- **Environment-aware**: Adapts configuration based on deployment environments (Local, Dev, Staging, Production)
- **Environment variable integration**: Automatically loads values from environment variables
- **Sensible defaults**: Comes with production-ready default configurations
- **Type safety**: Strong Rust types for all configuration parameters
- **Extensibility**: Customize with application-specific configurations via the `DynamicConfigs` trait
- **No external dependencies**: Minimal dependency footprint for your projects

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
configs = "0.1.0"
```

## Usage Examples

### Basic Configuration

```rust
use configs::{Configs, Empty};

fn main() {
    // Create configuration with default values
    let config = Configs::<Empty>::default();
    
    // Access configuration components
    println!("App will listen on {}", config.app.app_addr());
    println!("Environment: {}", config.app.env);
    
    // Check environment-specific behavior
    if config.app.env.is_prod() {
        println!("Running in production mode");
    }
}
```

### Loading from Environment Variables

```rust
use configs::{Configs, Empty};

fn main() {
    // Set environment variables (in a real app, these would come from the system)
    std::env::set_var("APP_PORT", "8080");
    std::env::set_var("POSTGRES_HOST", "db.example.com");
    
    // Load configuration from environment variables
    let config = Configs::<Empty>::new();
    
    // Values will reflect environment settings
    assert_eq!(config.app.port, 8080);
    assert_eq!(config.postgres.host, "db.example.com");
}
```

### Custom Application Configuration

Extend with your own application-specific settings:

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
        // Load from environment variables
        self.feature_flag_enabled = env::var("FEATURE_FLAG")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);
            
        self.max_connections = env::var("MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
    }
}

fn main() {
    // Create configs with custom dynamic configuration
    let mut config = Configs::<MyAppConfig>::default();
    
    // Load application-specific configuration
    config.dynamic.load();
    
    if config.dynamic.feature_flag_enabled {
        println!("Feature flag is enabled!");
    }
}
```

### Working with databases

```rust
use configs::{Configs, Empty, PostgresSslMode};

fn main() {
    std::env::set_var("APP_PORT", "8080");
    std::env::set_var("POSTGRES_HOST", "db.example.com");
    std::env::set_var("POSTGRES_PORT", "5432");
    std::env::set_var("POSTGRES_USER", "postgres");
    std::env::set_var("POSTGRES_PASSWORD", "postgres");
    std::env::set_var("POSTGRES_DB", "postgres");
    
    let mut config = Configs::<Empty>::new();
    
    // Use in your database client
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode={}",
        config.postgres.user,
        config.postgres.password,
        config.postgres.host,
        config.postgres.port,
        config.postgres.db,
        config.postgres.ssl_mode.to_string().to_lowercase()
    );
    
    println!("Connection string: {}", connection_string);
}
```

## Available Configuration Modules

- **[`app`](src/app.rs)**: Core application configuration (name, environment, host, port)
- **[`aws`](src/aws.rs)**: AWS credentials and region configuration
- **[`dynamic`](src/dynamic.rs)**: Interface for application-specific configuration extensions
- **[`dynamo`](src/dynamo.rs)**: Amazon DynamoDB configuration
- **[`environment`](src/environment.rs)**: Environment type (Local, Dev, Staging, Prod) and detection
- **[`health_readiness`](src/health_readiness.rs)**: Health and readiness probes configuration
- **[`identity_server`](src/identity_server.rs)**: OAuth/OpenID Connect identity provider configuration
- **[`influx`](src/influx.rs)**: InfluxDB time-series database configuration
- **[`kafka`](src/kafka.rs)**: Apache Kafka message broker configuration
- **[`mqtt`](src/mqtt.rs)**: MQTT message broker configuration
- **[`otlp`](src/otlp.rs)**: OpenTelemetry (OTLP) observability configuration
- **[`postgres`](src/postgres.rs)**: PostgreSQL database configuration
- **[`rabbitmq`](src/rabbitmq.rs)**: RabbitMQ message broker configuration
- **[`secrets`](src/secrets.rs)**: Secret management configuration
- **[`sqlite`](src/sqlite.rs)**: SQLite database configuration

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests.

## License

This project is licensed under the MIT License - see the [`LICENSE`](LICENSE) file for details.