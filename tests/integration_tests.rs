use reverdns::{DnsResolver, OutputFormat};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_dns_resolver_creation() {
    let resolver = DnsResolver::new(5).await;
    assert!(resolver.is_ok());
}

#[tokio::test]
async fn test_dns_resolver_with_custom_ip() {
    let resolver = DnsResolver::with_resolver("8.8.8.8", 5).await;
    assert!(resolver.is_ok());
}

#[tokio::test]
async fn test_dns_resolver_invalid_ip() {
    let resolver = DnsResolver::with_resolver("invalid", 5).await;
    assert!(resolver.is_err());
}

#[tokio::test]
async fn test_lookup_invalid_ip() {
    let resolver = DnsResolver::new(5).await.unwrap();
    let result = resolver.lookup("not-an-ip").await;
    assert!(result.is_err());
}

#[test]
fn test_output_format_json() {
    let format = OutputFormat::Json;
    assert_eq!(format.to_string(), "json");
}

#[test]
fn test_output_format_csv() {
    let format = OutputFormat::Csv;
    assert_eq!(format.to_string(), "csv");
}

#[test]
fn test_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Write test file
    fs::write(&file_path, "8.8.8.8\n1.1.1.1\n").unwrap();

    // Read test file
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("8.8.8.8"));
    assert!(content.contains("1.1.1.1"));
}

#[test]
fn test_json_output_format() {
    use reverdns::{LookupResult, LookupStatus};
    use reverdns::output::format_json;

    let results = vec![
        LookupResult {
            ip: "8.8.8.8".to_string(),
            hostname: Some("dns.google".to_string()),
            status: LookupStatus::Success,
            ttl: Some(3600),
            latency_ms: 45,
            resolver: "8.8.8.8".to_string(),
            error: None,
        },
    ];

    let json = format_json(&results, 100).unwrap();
    assert!(json.contains("8.8.8.8"));
    assert!(json.contains("dns.google"));
    assert!(json.contains("success"));
    assert!(json.contains("metadata"));
}

#[test]
fn test_csv_output_format() {
    use reverdns::{LookupResult, LookupStatus};
    use reverdns::output::format_csv;

    let results = vec![
        LookupResult {
            ip: "8.8.8.8".to_string(),
            hostname: Some("dns.google".to_string()),
            status: LookupStatus::Success,
            ttl: Some(3600),
            latency_ms: 45,
            resolver: "8.8.8.8".to_string(),
            error: None,
        },
    ];

    let csv = format_csv(&results).unwrap();
    assert!(csv.contains("8.8.8.8"));
    assert!(csv.contains("dns.google"));
    assert!(csv.contains("success"));
}
