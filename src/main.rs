use clap::Parser;
use colored::*;
use futures::{stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reverdns::{
    cli::Args,
    dns::DnsResolver,
    error::Result,
    logger,
    output::{format_csv, format_json},
};
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::{Duration, Instant};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Initialize logger
    if let Err(e) = logger::init_logger(&args.log_level.to_string()) {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }

    print_banner();

    // Validate arguments
    if let Err(e) = args.validate() {
        error!("Invalid arguments: {}", e);
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(e.exit_code());
    }

    // Run the application
    if let Err(e) = run(args).await {
        error!("Application error: {}", e);
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(e.exit_code());
    }
}

fn print_banner() {
    let banner = r#"
    ____                      ____  _   ______
   / __ \___ _   _____  _____/ __ \/ | / / ...
  / /_/ / _ \ | / / _ \/ ___/ / / /  |/ / /__ 
 / _, _/  __/ |/ /  __/ /  / /_/ / /|  /___  /
/_/ |_|\___/|___/\___/_/  /_____/_/ |_//____/ 
                                               
"#;
    eprintln!("{}", banner.cyan().bold());
    eprintln!(
        "{} v{}",
        "ReverDNS".green().bold(),
        env!("CARGO_PKG_VERSION")
    );
    eprintln!(
        "{}",
        "High-performance reverse DNS lookup tool".blue().italic()
    );
    eprintln!();
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
    eprintln!(
        "{} {} IP addresses to process",
        "ℹ".blue().bold(),
        ips.len()
    );

    // Create DNS resolver
    let resolver = if !args.resolver.is_empty() || args.dns_over_https {
        if !args.resolver.is_empty() {
            info!("Using custom resolvers: {:?}", args.resolver);
            eprintln!("{} Using custom resolvers", "ℹ".blue().bold());
        }
        if args.dns_over_https {
            info!("Using DNS-over-HTTPS");
            eprintln!(
                "{} Using DNS-over-HTTPS ({})",
                "ℹ".blue().bold(),
                args.doh_provider.clone().unwrap_or_default()
            );
        }

        DnsResolver::with_resolvers(
            &args.resolver,
            args.timeout,
            args.retry_count,
            args.retry_backoff,
            args.dns_over_https,
            args.doh_provider,
        )
        .await?
    } else {
        info!("Using default resolvers");
        DnsResolver::new(args.timeout, args.retry_count, args.retry_backoff).await?
    };

    // Calculate rate limit interval
    let rate_limit_interval = if args.rate_limit > 0 {
        Duration::from_secs_f64(1.0 / args.rate_limit as f64)
    } else {
        Duration::from_micros(1) // Practically no limit
    };

    let resolver_ref = &resolver;

    // Initialize Progress Bar
    let pb = ProgressBar::new(ips.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));

    // Concurrent processing loop
    let results = stream::iter(ips)
        .map(|ip| {
            let pb = pb.clone();
            async move {
                // Apply rate limit delay (simple approximation)
                tokio::time::sleep(rate_limit_interval).await;
                let result = resolver_ref.lookup(&ip).await;
                pb.inc(1);
                result
            }
        })
        .buffer_unordered(args.concurrency)
        .collect::<Vec<_>>()
        .await;

    pb.finish_with_message("Done");

    // Unwrap results
    let results: Vec<reverdns::LookupResult> = results
        .into_iter()
        .filter_map(|r| match r {
            Ok(res) => Some(res),
            Err(e) => {
                error!("Unexpected error type in stream: {}", e);
                None
            }
        })
        .collect();

    let elapsed = start_time.elapsed().as_millis();

    // Format output
    let output = match args.format {
        reverdns::OutputFormat::Json => format_json(&results, elapsed)?,
        reverdns::OutputFormat::Csv => format_csv(&results)?,
    };

    // Write output
    if let Some(output_file) = &args.output {
        info!("Writing results to file: {}", output_file);
        eprintln!(
            "{} Writing results to {}",
            "✔".green().bold(),
            output_file.white()
        );
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
                // Simple IP validation could go here, but we let the resolver handle strict parsing
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

    println!("\n{}", "=== Statistics ===".yellow().bold());
    println!("Total lookups: {}", results.len().to_string().cyan());
    println!("Successful:    {}", successful.to_string().green());
    println!("Failed:        {}", failed.to_string().red());

    if !results.is_empty() {
        let rate = (successful as f64 / results.len() as f64) * 100.0;
        let color_rate = if rate > 90.0 {
            rate.to_string().green()
        } else if rate > 50.0 {
            rate.to_string().yellow()
        } else {
            rate.to_string().red()
        };
        println!("Success rate:  {}%", color_rate);
    }

    println!("Total time:    {}ms", total_time_ms);
    println!("Avg latency:   {:.2}ms", avg_latency);

    if total_time_ms > 0 {
        println!(
            "Throughput:    {:.2} lookups/sec",
            (results.len() as f64 / total_time_ms as f64) * 1000.0
        );
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
