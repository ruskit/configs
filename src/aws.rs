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
    /// AWS access key ID (Default: "local")
    pub access_key_id: Option<String>,
    /// AWS secret access key (Default: "local")
    pub secret_access_key: Option<String>,
    /// AWS session token (Default: None)
    pub session_token: Option<String>,
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
