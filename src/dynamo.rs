// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for Amazon DynamoDB.
//!
//! This module provides configuration options for connecting to
//! and working with Amazon DynamoDB.

/// # DynamoConfigs
///
/// Configuration structure for Amazon DynamoDB.
///
/// This structure defines the connection parameters and settings for working
/// with Amazon DynamoDB tables. It includes endpoint configuration, region selection,
/// table name, and item expiration settings.
///
/// ## Examples
///
/// ```
/// use configs::DynamoConfigs;
///
/// let dynamo_config = DynamoConfigs::default();
/// // Use DynamoDB configuration with AWS SDK
/// ```
#[derive(Debug, Clone)]
pub struct DynamoConfigs {
    /// ENV KEY: "DYNAMO_ENDPOINT"
    ///
    /// The DynamoDB endpoint URL (Default: "localhost")
    pub endpoint: String,
    /// ENV KEY: "DYNAMO_REGION"
    ///
    /// The AWS region for DynamoDB (Default: "us-east-1")
    pub region: String,
    /// ENV KEY: "DYNAMO_TABLE"
    ///
    /// The name of the DynamoDB table (Default: "")
    pub table: String,
    /// ENV KEY: "DYNAMO_EXPIRE"
    ///
    /// The default time-to-live (TTL) for items in seconds (Default: 31536000 - 1 year)
    pub expire: u64,
}

pub const DYNAMO_ENDPOINT_ENV_KEY: &str = "DYNAMO_ENDPOINT";
pub const DYNAMO_TABLE_ENV_KEY: &str = "DYNAMO_TABLE";
pub const DYNAMO_REGION_ENV_KEY: &str = "DYNAMO_REGION";
pub const DYNAMO_EXPIRE_ENV_KEY: &str = "DYNAMO_EXPIRE";

impl DynamoConfigs {
    /// Creates a new `DynamoConfigs` instance from environments variables.
    ///
    /// This method initializes the DynamoDB configuration from environments variables
    /// for endpoint, region, table name, and item expiration settings.
    ///
    /// ## Returns
    ///
    /// A new `DynamoConfigs` from environments variables.
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.endpoint = std::env::var(DYNAMO_ENDPOINT_ENV_KEY).unwrap_or(cfgs.endpoint);
        cfgs.region = std::env::var(DYNAMO_REGION_ENV_KEY).unwrap_or(cfgs.region);
        cfgs.table = std::env::var(DYNAMO_TABLE_ENV_KEY).unwrap_or(cfgs.table);
        cfgs.expire = std::env::var(DYNAMO_EXPIRE_ENV_KEY)
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(cfgs.expire);

        cfgs
    }
}

impl Default for DynamoConfigs {
    fn default() -> Self {
        Self {
            endpoint: "localhost".to_owned(),
            region: "us-east-1".to_owned(),
            table: Default::default(),
            expire: 31536000,
        }
    }
}
