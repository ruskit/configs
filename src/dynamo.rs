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
    /// The DynamoDB endpoint URL (Default: "localhost")
    pub endpoint: String,
    /// The AWS region for DynamoDB (Default: "us-east-1")
    pub region: String,
    /// The name of the DynamoDB table (Default: "")
    pub table: String,
    /// The default time-to-live (TTL) for items in seconds (Default: 31536000 - 1 year)
    pub expire: u64,
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
