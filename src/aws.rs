// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! Defines configuration structures for AWS services.
//!
//! This module provides configuration options for authenticating with
//! and connecting to AWS services.

/// # AwsConfigs
///
/// Configuration structure for AWS service authentication.
///
/// This structure defines the AWS credentials needed to authenticate
/// with various AWS services like S3, DynamoDB, SecretsManager, etc.
///
/// ## Examples
///
/// ```
/// use configs::AwsConfigs;
///
/// let aws_config = AwsConfigs::default();
/// // Use AWS configuration with AWS SDK
/// ```
#[derive(Debug, Clone)]
pub struct AwsConfigs {
    /// ENV KEY: "AWS_IAM_ACCESS_KEY_ID"
    ///
    /// AWS access key ID (Default: "local")
    pub access_key_id: Option<String>,
    /// ENV KEY: "AWS_IAM_SECRET_ACCESS_KEY"
    ///
    /// AWS secret access key (Default: "local")
    pub secret_access_key: Option<String>,
    /// ENV KEY:
    ///
    /// AWS session token (Default: None)
    pub session_token: Option<String>,
}

pub const AWS_DEFAULT_REGION: &str = "us-east-1";
pub const AWS_IAM_ACCESS_KEY_ID: &str = "AWS_IAM_ACCESS_KEY_ID";
pub const AWS_IAM_SECRET_ACCESS_KEY: &str = "AWS_IAM_SECRET_ACCESS_KEY";

impl AwsConfigs {
    /// Creates a new `AwsConfigs` instance with environments variables.
    ///
    /// This method initializes the AWS configuration with environments variables
    /// for access key ID, secret access key, and session token.
    ///
    /// ## Returns
    ///
    /// A new `AwsConfigs` with environment variables.
    pub fn new() -> Self {
        let mut cfgs = Self::default();

        cfgs.access_key_id = std::env::var(AWS_IAM_ACCESS_KEY_ID)
            .ok()
            .or_else(|| std::env::var("AWS_ACCESS_KEY_ID").ok());
        cfgs.secret_access_key = std::env::var(AWS_IAM_SECRET_ACCESS_KEY)
            .ok()
            .or_else(|| std::env::var("AWS_SECRET_ACCESS_KEY").ok());

        cfgs
    }
}

impl Default for AwsConfigs {
    fn default() -> Self {
        Self {
            access_key_id: Some("local".to_owned()),
            secret_access_key: Some("local".to_owned()),
            session_token: Default::default(),
        }
    }
}
