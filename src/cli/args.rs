use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    #[value(name = "json")]
    Json,
    #[value(name = "csv")]
    Csv,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Csv => write!(f, "csv"),
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum LogLevel {
    #[value(name = "trace")]
    Trace,
    #[value(name = "debug")]
    Debug,
    #[value(name = "info")]
    Info,
    #[value(name = "warn")]
    Warn,
    #[value(name = "error")]
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

/// ReverDNS - High-performance reverse DNS lookup tool
#[derive(Parser, Debug)]
#[command(name = "reverdns")]
#[command(version = "2.0.0")]
#[command(about = "High-performance reverse DNS (PTR) lookup tool for bulk IP lists", long_about = None)]
pub struct Args {
    /// IP addresses to lookup (can be provided as arguments or via --input)
    #[arg(value_name = "IPS")]
    pub ips: Vec<String>,

    /// Input file with IP addresses (one per line)
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<String>,

    /// Output file path (default: stdout)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<String>,

    /// Output format: json, csv
    #[arg(short, long, value_enum, default_value = "json")]
    pub format: OutputFormat,

    /// Custom DNS resolver IP (can be used multiple times)
    #[arg(short, long, value_name = "RESOLVER")]
    pub resolver: Vec<String>,

    /// Timeout per lookup in seconds
    #[arg(short, long, value_name = "SECONDS", default_value = "5")]
    pub timeout: u64,

    /// Number of concurrent lookups
    #[arg(short, long, value_name = "NUM", default_value = "10")]
    pub concurrency: usize,

    /// Rate limit: lookups per second
    #[arg(short, long, value_name = "PER_SEC", default_value = "100")]
    pub rate_limit: u32,

    /// Use DNS-over-HTTPS (DoH)
    #[arg(long)]
    pub dns_over_https: bool,

    /// Custom DoH provider URL
    #[arg(long, value_name = "URL")]
    pub doh_provider: Option<String>,

    /// Number of retries on failure
    #[arg(long, value_name = "NUM", default_value = "3")]
    pub retry_count: u32,

    /// Initial backoff in milliseconds
    #[arg(long, value_name = "MS", default_value = "100")]
    pub retry_backoff: u64,

    /// Log level: trace, debug, info, warn, error
    #[arg(long, value_enum, default_value = "info")]
    pub log_level: LogLevel,

    /// Print statistics after completion
    #[arg(long)]
    pub stats: bool,

    /// Start web API server
    #[arg(long)]
    pub web_server: bool,

    /// Web server port
    #[arg(long, value_name = "PORT", default_value = "8080")]
    pub web_port: u16,
}

impl Args {
    /// Validate arguments
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.ips.is_empty() && self.input.is_none() {
            return Err(crate::error::ReverDNSError::ConfigError(
                "Either provide IPs as arguments or use --input flag".to_string(),
            ));
        }

        if self.concurrency == 0 {
            return Err(crate::error::ReverDNSError::ConfigError(
                "Concurrency must be greater than 0".to_string(),
            ));
        }

        if self.rate_limit == 0 {
            return Err(crate::error::ReverDNSError::ConfigError(
                "Rate limit must be greater than 0".to_string(),
            ));
        }

        if self.timeout == 0 {
            return Err(crate::error::ReverDNSError::ConfigError(
                "Timeout must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_display() {
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Csv.to_string(), "csv");
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Debug.to_string(), "debug");
        assert_eq!(LogLevel::Info.to_string(), "info");
    }
}
