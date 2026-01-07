use thiserror::Error;
use std::io;

/// Result type for ReverDNS operations
pub type Result<T> = std::result::Result<T, ReverDNSError>;

/// Comprehensive error types for ReverDNS
#[derive(Error, Debug)]
pub enum ReverDNSError {
    #[error("DNS resolution failed: {0}")]
    ResolutionFailed(String),

    #[error("Invalid IP address: {0}")]
    InvalidIpAddress(String),

    #[error("Timeout during DNS lookup")]
    Timeout,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("Invalid resolver: {0}")]
    InvalidResolver(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Retry limit exceeded")]
    RetryLimitExceeded,

    #[error("DNS-over-HTTPS error: {0}")]
    DoHError(String),

    #[error("Invalid output format: {0}")]
    InvalidFormat(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl ReverDNSError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            ReverDNSError::Timeout
                | ReverDNSError::NetworkError(_)
                | ReverDNSError::RateLimitExceeded
        )
    }

    /// Get error code for exit status
    pub fn exit_code(&self) -> i32 {
        match self {
            ReverDNSError::InvalidIpAddress(_) => 1,
            ReverDNSError::ConfigError(_) => 2,
            ReverDNSError::FileNotFound(_) => 3,
            ReverDNSError::PermissionDenied(_) => 4,
            ReverDNSError::Timeout => 5,
            ReverDNSError::NetworkError(_) => 6,
            ReverDNSError::RetryLimitExceeded => 7,
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_is_retryable() {
        assert!(ReverDNSError::Timeout.is_retryable());
        assert!(ReverDNSError::NetworkError("test".to_string()).is_retryable());
        assert!(!ReverDNSError::InvalidIpAddress("test".to_string()).is_retryable());
    }

    #[test]
    fn test_error_exit_code() {
        assert_eq!(ReverDNSError::InvalidIpAddress("test".to_string()).exit_code(), 1);
        assert_eq!(ReverDNSError::Timeout.exit_code(), 5);
        assert_eq!(ReverDNSError::FileNotFound("test".to_string()).exit_code(), 3);
    }
}
