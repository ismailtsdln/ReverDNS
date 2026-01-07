use crate::error::Result;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize logging with specified level
pub fn init_logger(level: &str) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true),
        )
        .try_init()
        .map_err(|e| crate::error::ReverDNSError::InternalError(e.to_string()))
        .unwrap_or(()); // Ignore error if logger is already initialized

    Ok(())
}

/// Initialize JSON logging for structured output
pub fn init_json_logger(level: &str) -> Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt::layer().json().with_writer(std::io::stderr))
        .try_init()
        .map_err(|e| crate::error::ReverDNSError::InternalError(e.to_string()))
        .unwrap_or(()); // Ignore error if logger is already initialized

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_logger() {
        let result = init_logger("debug");
        assert!(result.is_ok());
    }

    #[test]
    fn test_init_json_logger() {
        let result = init_json_logger("info");
        assert!(result.is_ok());
    }
}
