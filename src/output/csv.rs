use crate::dns::LookupResult;
use crate::error::Result;
use csv::Writer;
use serde::Serialize;
use chrono::Utc;

#[derive(Debug, Serialize)]
pub struct CsvRecord {
    pub ip: String,
    pub hostname: String,
    pub status: String,
    pub ttl: String,
    pub latency_ms: u128,
    pub resolver: String,
    pub error: String,
    pub timestamp: String,
}

/// Format lookup results as CSV
pub fn format_csv(results: &[LookupResult]) -> Result<String> {
    let mut wtr = Writer::from_writer(vec![]);

    // Write header
    wtr.write_record(&[
        "ip",
        "hostname",
        "status",
        "ttl",
        "latency_ms",
        "resolver",
        "error",
        "timestamp",
    ])?;

    // Write records
    for result in results {
        let record = CsvRecord {
            ip: result.ip.clone(),
            hostname: result.hostname.clone().unwrap_or_default(),
            status: result.status.to_string(),
            ttl: result.ttl.map(|t| t.to_string()).unwrap_or_default(),
            latency_ms: result.latency_ms,
            resolver: result.resolver.clone(),
            error: result.error.clone().unwrap_or_default(),
            timestamp: Utc::now().to_rfc3339(),
        };

        wtr.serialize(record)?;
    }

    wtr.flush()?;
    let data = wtr.into_inner()?;
    Ok(String::from_utf8(data)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dns::LookupStatus;

    #[test]
    fn test_format_csv() {
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

    #[test]
    fn test_format_csv_empty() {
        let results = vec![];
        let csv = format_csv(&results).unwrap();
        assert!(csv.contains("ip,hostname,status"));
    }
}
