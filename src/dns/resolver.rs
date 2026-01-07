use crate::error::{Result, ReverDNSError};
use std::net::IpAddr;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tracing::{debug, error, warn};
use trust_dns_resolver::config::*;
use trust_dns_resolver::proto::rr::RecordType;
use trust_dns_resolver::TokioAsyncResolver;

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
    retry_count: u32,
    retry_backoff: Duration,
    resolver_names: String, // String representation for logging
}

impl DnsResolver {
    /// Create a new DNS resolver with default settings
    pub async fn new(timeout_secs: u64, retry_count: u32, retry_backoff_ms: u64) -> Result<Self> {
        // Use Google DNS and Cloudflare as defaults if system config fails or for consistency
        let mut config = ResolverConfig::default();
        config.add_name_server(NameServerConfig {
            socket_addr: SocketAddr::new(IpAddr::V4("8.8.8.8".parse().unwrap()), 53),
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_negative_responses: true,
            bind_addr: None,
        });
        config.add_name_server(NameServerConfig {
            socket_addr: SocketAddr::new(IpAddr::V4("1.1.1.1".parse().unwrap()), 53),
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_negative_responses: true,
            bind_addr: None,
        });

        let mut opts = ResolverOpts::default();
        opts.timeout = Duration::from_secs(timeout_secs);
        opts.attempts = 1; // We handle retries manually for better control

        let resolver = TokioAsyncResolver::tokio(config, opts);

        Ok(Self {
            resolver,
            timeout: Duration::from_secs(timeout_secs),
            retry_count,
            retry_backoff: Duration::from_millis(retry_backoff_ms),
            resolver_names: "default(8.8.8.8,1.1.1.1)".to_string(),
        })
    }

    /// Create a new DNS resolver with custom resolver IPs
    pub async fn with_resolvers(
        resolver_ips: &[String],
        timeout_secs: u64,
        retry_count: u32,
        retry_backoff_ms: u64,
        use_doh: bool,
        doh_provider: Option<String>,
    ) -> Result<Self> {
        let (config, resolver_names) = if use_doh {
            let provider_url =
                doh_provider.unwrap_or_else(|| "https://cloudflare-dns.com/dns-query".to_string());

            let mut config = ResolverConfig::new();
            let mut names = String::new();

            if provider_url.contains("cloudflare") {
                config.add_name_server(NameServerConfig {
                    socket_addr: SocketAddr::new(IpAddr::V4("1.1.1.1".parse().unwrap()), 443),
                    protocol: Protocol::Https,
                    tls_dns_name: Some("cloudflare-dns.com".to_string()),
                    trust_negative_responses: true,
                    bind_addr: None,
                });
                names = "cloudflare-doh".to_string();
            } else if provider_url.contains("google") {
                config.add_name_server(NameServerConfig {
                    socket_addr: SocketAddr::new(IpAddr::V4("8.8.8.8".parse().unwrap()), 443),
                    protocol: Protocol::Https,
                    tls_dns_name: Some("dns.google".to_string()),
                    trust_negative_responses: true,
                    bind_addr: None,
                });
                names = "google-doh".to_string();
            } else if provider_url.contains("quad9") {
                config.add_name_server(NameServerConfig {
                    socket_addr: SocketAddr::new(IpAddr::V4("9.9.9.9".parse().unwrap()), 5053),
                    protocol: Protocol::Https,
                    tls_dns_name: Some("dns.quad9.net".to_string()),
                    trust_negative_responses: true,
                    bind_addr: None,
                });
                names = "quad9-doh".to_string();
            } else {
                // Fallback to cloudflare for generic default
                config.add_name_server(NameServerConfig {
                    socket_addr: SocketAddr::new(IpAddr::V4("1.1.1.1".parse().unwrap()), 443),
                    protocol: Protocol::Https,
                    tls_dns_name: Some("cloudflare-dns.com".to_string()),
                    trust_negative_responses: true,
                    bind_addr: None,
                });
                names = "cloudflare-doh(fallback)".to_string();
            }
            (config, names)
        } else {
            let mut config = ResolverConfig::new();
            let mut names = Vec::new();

            for ip_str in resolver_ips {
                let ip_addr = IpAddr::from_str(ip_str)
                    .map_err(|_| ReverDNSError::InvalidResolver(ip_str.to_string()))?;

                config.add_name_server(NameServerConfig {
                    socket_addr: SocketAddr::new(ip_addr, 53),
                    protocol: Protocol::Udp,
                    tls_dns_name: None,
                    trust_negative_responses: true,
                    bind_addr: None,
                });
                names.push(ip_str.clone());
            }
            (config, names.join(","))
        };

        let mut opts = ResolverOpts::default();
        opts.timeout = Duration::from_secs(timeout_secs);
        opts.attempts = 1;

        let resolver = TokioAsyncResolver::tokio(config, opts);

        Ok(Self {
            resolver,
            timeout: Duration::from_secs(timeout_secs),
            retry_count,
            retry_backoff: Duration::from_millis(retry_backoff_ms),
            resolver_names,
        })
    }

    /// Perform reverse DNS lookup for an IP address
    pub async fn lookup(&self, ip: &str) -> Result<LookupResult> {
        let start = std::time::Instant::now();

        // Validate IP address
        let ip_addr =
            IpAddr::from_str(ip).map_err(|_| ReverDNSError::InvalidIpAddress(ip.to_string()))?;

        debug!("Looking up IP: {}", ip);

        // Perform lookup with retry logic
        let mut last_error = None;

        for attempt in 0..=self.retry_count {
            if attempt > 0 {
                tokio::time::sleep(self.retry_backoff * attempt).await;
                debug!(
                    "Retrying lookup for {} (attempt {}/{})",
                    ip, attempt, self.retry_count
                );
            }

            // Create reverse lookup query
            let result =
                tokio::time::timeout(self.timeout, self.resolver.reverse_lookup(ip_addr)).await;

            match result {
                Ok(Ok(lookup_result)) => {
                    let latency_ms = start.elapsed().as_millis();

                    // Taking the first hostname if available
                    let hostname = lookup_result.iter().next().map(|h| h.to_utf8());
                    // Remove trailing dot
                    let hostname = hostname.map(|h| h.trim_end_matches('.').to_string());

                    // TTL is not directly exposed in reverse_lookup simplified result in all trust-dns versions,
                    // but usually available if we used basic lookup. For now keeping None or 0.
                    let ttl: Option<u32> = None;

                    return Ok(LookupResult {
                        ip: ip.to_string(),
                        hostname,
                        status: LookupStatus::Success,
                        ttl,
                        latency_ms,
                        resolver: self.resolver_names.clone(),
                        error: None,
                    });
                }
                Ok(Err(e)) => {
                    last_error = Some(e.to_string());
                    // Should we retry or is it a hard failure (NXDOMAIN)?
                    // Typically NXDOMAIN is hard failure. Connection refused is retryable.
                    // Simplified: Retry everything except NXDOMAIN.
                    let err_str = e.to_string();
                    if err_str.contains("NXDOMAIN") || err_str.contains("NoRecordsFound") {
                        break;
                    }
                }
                Err(_) => {
                    last_error = Some("Timeout".to_string());
                }
            }
        }

        let latency_ms = start.elapsed().as_millis();
        let error_msg = last_error.unwrap_or_else(|| "Unknown error".to_string());

        let status = if error_msg.contains("Timeout") {
            LookupStatus::Timeout
        } else {
            LookupStatus::Failed
        };

        Ok(LookupResult {
            ip: ip.to_string(),
            hostname: None,
            status,
            ttl: None,
            latency_ms,
            resolver: self.resolver_names.clone(),
            error: Some(error_msg),
        })
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
        let resolver = DnsResolver::new(5, 1, 100).await;
        assert!(resolver.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_resolver_ip() {
        // This accepts string so we can test "invalid"
        let result =
            DnsResolver::with_resolvers(&vec!["invalid".to_string()], 5, 1, 100, false, None).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_ip_lookup() {
        let resolver = DnsResolver::new(5, 0, 0).await.unwrap();
        let result = resolver.lookup("invalid").await;
        assert!(result.is_err());
    }
}
