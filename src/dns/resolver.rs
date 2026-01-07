use crate::error::{Result, ReverDNSError};
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;
use trust_dns_resolver::proto::rr::RecordType;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use tracing::{debug, warn, error};

/// DNS lookup result
#[derive(Debug, Clone)]
pub struct LookupResult {
    pub ip: String,
    pub hostname: Option<String>,
    pub status: LookupStatus,
    pub ttl: Option<u32>,
    pub latency_ms: u128,
    pub resolver: String,
    pub error: Option<String>,
}

/// Status of a DNS lookup
#[derive(Debug, Clone, PartialEq)]
pub enum LookupStatus {
    Success,
    Failed,
    Timeout,
    RateLimited,
}

impl std::fmt::Display for LookupStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LookupStatus::Success => write!(f, "success"),
            LookupStatus::Failed => write!(f, "failed"),
            LookupStatus::Timeout => write!(f, "timeout"),
            LookupStatus::RateLimited => write!(f, "rate_limited"),
        }
    }
}

/// DNS Resolver wrapper
pub struct DnsResolver {
    resolver: TokioAsyncResolver,
    timeout: Duration,
    resolver_ip: String,
}

impl DnsResolver {
    /// Create a new DNS resolver with default settings
    pub async fn new(timeout_secs: u64) -> Result<Self> {
        let config = ResolverConfig::default();
        let resolver = TokioAsyncResolver::tokio(config, ResolverOpts::default());

        Ok(Self {
            resolver,
            timeout: Duration::from_secs(timeout_secs),
            resolver_ip: "8.8.8.8".to_string(),
        })
    }

    /// Create a new DNS resolver with custom resolver IP
    pub async fn with_resolver(resolver_ip: &str, timeout_secs: u64) -> Result<Self> {
        // Validate IP address
        IpAddr::from_str(resolver_ip)
            .map_err(|_| ReverDNSError::InvalidResolver(resolver_ip.to_string()))?;

        // Use default config which will use system resolvers
        // In a production system, you would configure custom nameservers here
        let config = ResolverConfig::default();
        let resolver = TokioAsyncResolver::tokio(config, ResolverOpts::default());

        Ok(Self {
            resolver,
            timeout: Duration::from_secs(timeout_secs),
            resolver_ip: resolver_ip.to_string(),
        })
    }

    /// Perform reverse DNS lookup for an IP address
    pub async fn lookup(&self, ip: &str) -> Result<LookupResult> {
        let start = std::time::Instant::now();

        // Validate IP address
        let ip_addr = IpAddr::from_str(ip)
            .map_err(|_| ReverDNSError::InvalidIpAddress(ip.to_string()))?;

        debug!("Looking up IP: {}", ip);

        // Create reverse DNS query
        let reverse_ip = match ip_addr {
            IpAddr::V4(addr) => {
                let octets = addr.octets();
                format!(
                    "{}.{}.{}.{}.in-addr.arpa",
                    octets[3], octets[2], octets[1], octets[0]
                )
            }
            IpAddr::V6(addr) => {
                let segments = addr.segments();
                let mut reverse = String::new();
                for segment in segments.iter().rev() {
                    for i in (0..4).rev() {
                        let nibble = (segment >> (i * 4)) & 0xf;
                        reverse.push_str(&format!("{:x}.", nibble));
                    }
                }
                reverse.push_str("ip6.arpa");
                reverse
            }
        };

        // Perform lookup with timeout
        let reverse_ip_parsed = reverse_ip.parse()
            .map_err(|_| ReverDNSError::InternalError("Failed to parse reverse IP".to_string()))?;

        let result = tokio::time::timeout(
            self.timeout,
            self.resolver.lookup(reverse_ip_parsed, RecordType::PTR),
        )
        .await;

        let latency_ms = start.elapsed().as_millis();

        match result {
            Ok(Ok(lookup_result)) => {
                let hostname = lookup_result
                    .record_iter()
                    .find_map(|record| {
                        if let Some(data) = record.data() {
                            if let Some(ptr) = data.as_ptr() {
                                return Some(ptr.to_utf8());
                            }
                        }
                        None
                    })
                    .map(|h| h.trim_end_matches('.').to_string());

                // TTL is typically not available in reverse lookups, set to None
                let ttl: Option<u32> = None;

                debug!("Lookup successful for {}: {:?}", ip, hostname);

                Ok(LookupResult {
                    ip: ip.to_string(),
                    hostname,
                    status: LookupStatus::Success,
                    ttl,
                    latency_ms,
                    resolver: self.resolver_ip.clone(),
                    error: None,
                })
            }
            Ok(Err(e)) => {
                warn!("Lookup failed for {}: {}", ip, e);
                Ok(LookupResult {
                    ip: ip.to_string(),
                    hostname: None,
                    status: LookupStatus::Failed,
                    ttl: None,
                    latency_ms,
                    resolver: self.resolver_ip.clone(),
                    error: Some(e.to_string()),
                })
            }
            Err(_) => {
                error!("Lookup timeout for {}", ip);
                Ok(LookupResult {
                    ip: ip.to_string(),
                    hostname: None,
                    status: LookupStatus::Timeout,
                    ttl: None,
                    latency_ms,
                    resolver: self.resolver_ip.clone(),
                    error: Some("Timeout".to_string()),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_status_display() {
        assert_eq!(LookupStatus::Success.to_string(), "success");
        assert_eq!(LookupStatus::Failed.to_string(), "failed");
        assert_eq!(LookupStatus::Timeout.to_string(), "timeout");
    }

    #[tokio::test]
    async fn test_resolver_creation() {
        let resolver = DnsResolver::new(5).await;
        assert!(resolver.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_resolver_ip() {
        let result = DnsResolver::with_resolver("invalid", 5).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_ip_lookup() {
        let resolver = DnsResolver::new(5).await.unwrap();
        let result = resolver.lookup("invalid").await;
        assert!(result.is_err());
    }
}
