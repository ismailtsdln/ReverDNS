//! ReverDNS - High-performance reverse DNS lookup tool
//!
//! This library provides functionality for bulk reverse DNS (PTR) lookups with support for:
//! - JSON and CSV output formats
//! - Resolver rotation
//! - Rate limiting
//! - DNS-over-HTTPS
//! - Comprehensive error handling
//! - Async/await with Tokio

pub mod cli;
pub mod dns;
pub mod error;
pub mod logger;
pub mod output;

pub use error::{Result, ReverDNSError};
pub use dns::{DnsResolver, LookupResult, LookupStatus};
pub use cli::{Args, OutputFormat, LogLevel};
