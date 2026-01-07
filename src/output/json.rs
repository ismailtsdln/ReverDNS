use crate::dns::LookupResult;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonOutput {
    pub results: Vec<JsonResult>,
    pub metadata: JsonMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResult {
    pub ip: String,
    pub hostname: Option<String>,
    pub status: String,
    pub ttl: Option<u32>,
    pub latency_ms: u128,
    pub resolver: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonMetadata {
    pub total_lookups: usize,
    pub successful: usize,
    pub failed: usize,
    pub total_time_ms: u128,
    pub average_latency_ms: f64,
}

/// Format lookup results as JSON
pub fn format_json(results: &[LookupResult], total_time_ms: u128) -> Result<String> {
    use crate::dns::LookupStatus;

    let successful = results.iter().filter(|r| r.status == LookupStatus::Success).count();
    let failed = results.len() - successful;
    let average_latency = if results.is_empty() {
        0.0
    } else {
        results.iter().map(|r| r.latency_ms as f64).sum::<f64>() / results.len() as f64
    };

    let json_results: Vec<JsonResult> = results
        .iter()
        .map(|r| JsonResult {
            ip: r.ip.clone(),
            hostname: r.hostname.clone(),
            status: r.status.to_string(),
            ttl: r.ttl,
            latency_ms: r.latency_ms,
            resolver: r.resolver.clone(),
            error: r.error.clone(),
            timestamp: Utc::now().to_rfc3339(),
        })
        .collect();

    let output = JsonOutput {
        results: json_results,
        metadata: JsonMetadata {
            total_lookups: results.len(),
            successful,
            failed,
            total_time_ms,
            average_latency_ms: average_latency,
        },
    };

    Ok(serde_json::to_string_pretty(&output)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dns::LookupStatus;

    #[test]
    fn test_format_json() {
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
    }

    #[test]
    fn test_format_json_empty() {
        let results = vec![];
        let json = format_json(&results, 0).unwrap();
        assert!(json.contains("results"));
        assert!(json.contains("metadata"));
    }
}
