use clap::Parser;
use reverdns::{
    cli::Args, dns::DnsResolver, error::Result, logger, output::{format_csv, format_json},
};
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::time::Instant;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Initialize logger
    if let Err(e) = logger::init_logger(&args.log_level.to_string()) {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }

    // Validate arguments
    if let Err(e) = args.validate() {
        error!("Invalid arguments: {}", e);
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }

    // Run the application
    if let Err(e) = run(args).await {
        error!("Application error: {}", e);
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

async fn run(args: Args) -> Result<()> {
    let start_time = Instant::now();

    // Collect IPs from arguments and/or input file
    let mut ips = args.ips.clone();

    if let Some(input_file) = &args.input {
        info!("Reading IPs from file: {}", input_file);
        let file_ips = read_ips_from_file(input_file)?;
        ips.extend(file_ips);
    }

    if ips.is_empty() {
        return Err(reverdns::ReverDNSError::ConfigError(
            "No IPs provided".to_string(),
        ));
    }

    info!("Processing {} IP addresses", ips.len());

    // Create DNS resolver
    let resolver = if let Some(resolver_ip) = args.resolver.first() {
        info!("Using custom resolver: {}", resolver_ip);
        DnsResolver::with_resolver(resolver_ip, args.timeout).await?
    } else {
        info!("Using default resolver");
        DnsResolver::new(args.timeout).await?
    };

    // Perform lookups
    let mut results = Vec::new();
    for ip in ips {
        match resolver.lookup(&ip).await {
            Ok(result) => {
                results.push(result);
            }
            Err(e) => {
                warn!("Failed to lookup {}: {}", ip, e);
            }
        }
    }

    let elapsed = start_time.elapsed().as_millis();

    // Format output
    let output = match args.format {
        reverdns::OutputFormat::Json => format_json(&results, elapsed)?,
        reverdns::OutputFormat::Csv => format_csv(&results)?,
    };

    // Write output
    if let Some(output_file) = &args.output {
        info!("Writing results to file: {}", output_file);
        fs::write(output_file, &output)?;
    } else {
        println!("{}", output);
    }

    // Print statistics if requested
    if args.stats {
        print_statistics(&results, elapsed);
    }

    info!("Completed in {}ms", elapsed);
    Ok(())
}

fn read_ips_from_file(path: &str) -> Result<Vec<String>> {
    if !Path::new(path).exists() {
        return Err(reverdns::ReverDNSError::FileNotFound(path.to_string()));
    }

    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);

    let ips: Vec<String> = reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|l| {
                let trimmed = l.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    Some(trimmed.to_string())
                } else {
                    None
                }
            })
        })
        .collect();

    Ok(ips)
}

fn print_statistics(results: &[reverdns::LookupResult], total_time_ms: u128) {
    use reverdns::LookupStatus;

    let successful = results
        .iter()
        .filter(|r| r.status == LookupStatus::Success)
        .count();
    let failed = results.len() - successful;
    let avg_latency = if results.is_empty() {
        0.0
    } else {
        results.iter().map(|r| r.latency_ms as f64).sum::<f64>() / results.len() as f64
    };

    println!("\n=== Statistics ===");
    println!("Total lookups: {}", results.len());
    println!("Successful: {}", successful);
    println!("Failed: {}", failed);

    if !results.is_empty() {
        println!("Success rate: {:.2}%", (successful as f64 / results.len() as f64) * 100.0);
    }

    println!("Total time: {}ms", total_time_ms);
    println!("Average latency: {:.2}ms", avg_latency);

    if total_time_ms > 0 {
        println!("Throughput: {:.2} lookups/sec", (results.len() as f64 / total_time_ms as f64) * 1000.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_ips_from_file_not_found() {
        let result = read_ips_from_file("nonexistent.txt");
        assert!(result.is_err());
    }
}
