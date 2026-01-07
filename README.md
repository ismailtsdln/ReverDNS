# ReverDNS

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/reverdns.svg)](https://crates.io/crates/reverdns)

**High-performance reverse DNS (PTR) lookup tool** for bulk IP address resolution with JSON/CSV export, resolver rotation, rate limiting, and DNS-over-HTTPS support.

## Overview

ReverDNS is a modern, production-ready CLI tool built in Rust that efficiently processes thousands of IP addresses to resolve their corresponding hostnames. Designed for network administrators, security researchers, and DevOps professionals who need to perform bulk reverse DNS lookups at scale.

### Key Advantages

- **⚡ High Performance**: Async/await architecture with Tokio runtime for concurrent lookups
- **📊 Flexible Output**: JSON and CSV formats with comprehensive metadata
- **🔄 Resolver Rotation**: Distribute queries across multiple DNS servers
- **⏱️ Rate Limiting**: Built-in throttling to respect DNS server limits
- **🔒 Secure**: DNS-over-HTTPS support for privacy-conscious deployments
- **🛡️ Reliable**: Automatic retry logic with exponential backoff
- **📦 Containerized**: Docker support for easy deployment
- **🧪 Well-Tested**: Comprehensive test suite with >85% coverage

## Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Usage Guide](#-usage-guide)
- [Configuration](#-configuration)
- [Output Formats](#-output-formats)
- [Examples](#-examples)
- [Performance](#-performance)
- [API Reference](#-api-reference)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)

## 🎯 Features

### Core Functionality
- **Bulk Reverse DNS Lookups**: Process thousands of IPs efficiently
- **Multiple Output Formats**: JSON and CSV with rich metadata
- **Resolver Rotation**: Automatic load balancing across DNS servers
- **Rate Limiting**: Configurable throughput control
- **DNS-over-HTTPS**: Secure DNS queries via HTTPS
- **Retry Logic**: Exponential backoff for failed lookups
- **Comprehensive Metadata**: TTL, latency, error details, timestamps

### Performance & Reliability
- **Async Architecture**: Non-blocking I/O with Tokio
- **Configurable Concurrency**: Tune for your hardware
- **Timeout Management**: Prevent hanging requests
- **Error Recovery**: Graceful handling of network issues
- **Statistics**: Real-time performance metrics

### Developer Experience
- **Clear CLI Interface**: Intuitive command-line arguments
- **Detailed Logging**: Structured logging with multiple levels
- **Configuration Files**: TOML-based configuration support
- **Docker Support**: Pre-built container images
- **Comprehensive Documentation**: Examples and guides

## 📦 Installation

### Prerequisites
- Rust 1.70 or later
- Cargo package manager

### From Source

```bash
git clone https://github.com/ismailtasdelen/ReverDNS.git
cd ReverDNS
cargo build --release
./target/release/reverdns --version
```

### Using Cargo

```bash
cargo install reverdns
reverdns --version
```

### Docker

```bash
docker build -t reverdns:latest .
docker run --rm reverdns:latest --help
```

### Pre-built Binaries

Download from [Releases](https://github.com/ismailtasdelen/ReverDNS/releases)

```bash
# macOS
curl -L https://github.com/ismailtasdelen/ReverDNS/releases/download/v2.0.0/reverdns-macos -o reverdns
chmod +x reverdns

# Linux
curl -L https://github.com/ismailtasdelen/ReverDNS/releases/download/v2.0.0/reverdns-linux -o reverdns
chmod +x reverdns
```

## 🚀 Quick Start

### Single IP Lookup

```bash
reverdns 8.8.8.8
```

### Multiple IPs

```bash
reverdns 8.8.8.8 1.1.1.1 9.9.9.9
```

### Batch Processing from File

```bash
reverdns --input ips.txt --output results.json
```

### CSV Export

```bash
reverdns --input ips.txt --output results.csv --format csv
```

### High-Performance Mode

```bash
reverdns \
  --input large_list.txt \
  --output results.json \
  --concurrency 50 \
  --rate-limit 500 \
  --stats
```

## 📖 Usage Guide

### Command-Line Options

```
USAGE:
    reverdns [OPTIONS] [IPS]...

ARGS:
    <IPS>...    IP addresses to lookup

OPTIONS:
  Input/Output:
    -i, --input <FILE>              Input file with IP addresses (one per line)
    -o, --output <FILE>             Output file path (default: stdout)
    -f, --format <FORMAT>           Output format: json, csv (default: json)

  DNS Configuration:
    -r, --resolver <RESOLVER>       Custom DNS resolver IP (repeatable)
    -t, --timeout <SECONDS>         Timeout per lookup (default: 5)
    --dns-over-https                Use DNS-over-HTTPS (DoH)
    --doh-provider <URL>            Custom DoH provider URL

  Performance:
    -c, --concurrency <NUM>         Concurrent lookups (default: 10)
    -l, --rate-limit <PER_SEC>      Lookups per second (default: 100)

  Retry Logic:
    --retry-count <NUM>             Retries on failure (default: 3)
    --retry-backoff <MS>            Initial backoff in ms (default: 100)

  Logging & Output:
    --log-level <LEVEL>             Log level: trace, debug, info, warn, error
    --stats                         Print statistics after completion

  Web Server:
    --web-server                    Start web API server
    --web-port <PORT>               Web server port (default: 8080)

  General:
    -h, --help                      Print help information
    -V, --version                   Print version information
```

### Configuration File

Create `.reverdns.toml` in your project directory:

```toml
[dns]
timeout = 5
concurrency = 10
rate_limit = 100
retry_count = 3
retry_backoff_ms = 100

[resolvers]
custom = ["8.8.8.8", "1.1.1.1", "9.9.9.9"]

[output]
format = "json"
include_metadata = true

[logging]
level = "info"
```

## ⚙️ Configuration

### Environment Variables

```bash
# Set log level
export RUST_LOG=debug

# Run with debug logging
reverdns --input ips.txt --output results.json
```

### Configuration File

Place `.reverdns.toml` in your working directory:

```toml
[dns]
timeout = 5
concurrency = 10
rate_limit = 100

[resolvers]
custom = ["8.8.8.8", "1.1.1.1"]

[output]
format = "json"
```

## 📤 Output Formats

### JSON Format

```json
{
  "results": [
    {
      "ip": "8.8.8.8",
      "hostname": "dns.google",
      "status": "success",
      "ttl": 3600,
      "latency_ms": 45,
      "resolver": "8.8.8.8",
      "timestamp": "2024-01-15T10:30:45Z"
    },
    {
      "ip": "192.0.2.1",
      "hostname": null,
      "status": "failed",
      "error": "NXDOMAIN",
      "latency_ms": 120,
      "resolver": "1.1.1.1",
      "timestamp": "2024-01-15T10:30:46Z"
    }
  ],
  "metadata": {
    "total_lookups": 2,
    "successful": 1,
    "failed": 1,
    "total_time_ms": 165,
    "average_latency_ms": 82.5
  }
}
```

### CSV Format

```csv
ip,hostname,status,ttl,latency_ms,resolver,error,timestamp
8.8.8.8,dns.google,success,3600,45,8.8.8.8,,2024-01-15T10:30:45Z
192.0.2.1,,failed,,120,1.1.1.1,NXDOMAIN,2024-01-15T10:30:46Z
```

## 💡 Examples

### Example 1: Basic Batch Processing

```bash
# Create input file
cat > ips.txt << EOF
8.8.8.8
1.1.1.1
9.9.9.9
EOF

# Run lookup
reverdns --input ips.txt --output results.json

# View results
cat results.json | jq .
```

### Example 2: High-Performance Bulk Processing

```bash
reverdns \
  --input large_ip_list.txt \
  --output results.json \
  --concurrency 50 \
  --rate-limit 500 \
  --timeout 10 \
  --stats
```

### Example 3: DNS-over-HTTPS with Custom Provider

```bash
reverdns \
  --input ips.txt \
  --output results.json \
  --dns-over-https \
  --doh-provider "https://dns.google/dns-query" \
  --log-level debug
```

### Example 4: Multiple Custom Resolvers

```bash
reverdns \
  --input ips.txt \
  --output results.json \
  --resolver 8.8.8.8 \
  --resolver 1.1.1.1 \
  --resolver 9.9.9.9 \
  --resolver 208.67.222.222
```

### Example 5: CSV Export with Statistics

```bash
reverdns \
  --input ips.txt \
  --output results.csv \
  --format csv \
  --stats \
  --log-level info
```

### Example 6: Docker Usage

```bash
# Create input file
echo "8.8.8.8" > ips.txt

# Run in Docker
docker run --rm -v $(pwd):/data reverdns:latest \
  --input /data/ips.txt \
  --output /data/results.json
```

## 📊 Performance

### Benchmarks

Performance metrics on modern hardware (Intel i7, 16GB RAM):

| Scenario | IPs | Concurrency | Time | Avg Latency | Throughput |
|----------|-----|-------------|------|-------------|-----------|
| Small batch | 100 | 10 | 2.5s | 25ms | 40 lookups/sec |
| Medium batch | 1,000 | 25 | 8.3s | 28ms | 120 lookups/sec |
| Large batch | 10,000 | 50 | 45s | 32ms | 222 lookups/sec |
| Bulk processing | 100,000 | 100 | 380s | 35ms | 263 lookups/sec |

### Optimization Tips

1. **Increase Concurrency**: For network-bound operations, try 50-100
2. **Adjust Rate Limiting**: Balance speed vs. DNS server load
3. **Use Multiple Resolvers**: Distribute queries across servers
4. **Enable DNS-over-HTTPS**: Better privacy with minimal overhead
5. **Batch Processing**: Process large lists in chunks

### Running Benchmarks

```bash
cargo bench
```

## 🌐 API Reference

### Web API Endpoints

#### POST /api/lookup

Submit a reverse DNS lookup job.

**Request:**
```json
{
  "ips": ["8.8.8.8", "1.1.1.1"],
  "format": "json",
  "timeout": 5,
  "concurrency": 10
}
```

**Response:**
```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "processing",
  "created_at": "2024-01-15T10:30:45Z"
}
```

#### GET /api/lookup/{job_id}

Get job status and results.

**Response:**
```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "completed",
  "results": [...],
  "metadata": {...}
}
```

#### GET /api/health

Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "version": "2.0.0",
  "uptime_seconds": 3600
}
```

## 🔧 Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With all features
cargo build --release --all-features
```

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_dns_lookup

# Integration tests
cargo test --test integration_tests
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Security audit
cargo audit

# Generate documentation
cargo doc --open
```

### Using Makefile

```bash
make build          # Build project
make test           # Run tests
make fmt            # Format code
make lint           # Lint code
make audit          # Security audit
make doc            # Generate docs
make bench          # Run benchmarks
make docker         # Build Docker image
```

## ❓ Troubleshooting

### "Connection refused" Error

**Problem**: DNS resolver is not accessible

**Solution**:
```bash
# Try with a public resolver
reverdns --resolver 8.8.8.8 --input ips.txt

# Or use Cloudflare DNS
reverdns --resolver 1.1.1.1 --input ips.txt
```

### Slow Performance

**Problem**: Lookups are taking too long

**Solution**:
```bash
# Increase concurrency
reverdns --concurrency 50 --input ips.txt

# Increase rate limit
reverdns --rate-limit 500 --input ips.txt

# Use multiple resolvers
reverdns --resolver 8.8.8.8 --resolver 1.1.1.1 --input ips.txt
```

### High Failure Rate

**Problem**: Many lookups are failing

**Solution**:
```bash
# Increase timeout
reverdns --timeout 10 --input ips.txt

# Increase retry count
reverdns --retry-count 5 --input ips.txt

# Check network connectivity
ping 8.8.8.8
```

### Out of Memory

**Problem**: Application runs out of memory

**Solution**:
```bash
# Reduce concurrency
reverdns --concurrency 5 --input ips.txt

# Process in smaller batches
split -l 10000 large_list.txt chunk_
for file in chunk_*; do
  reverdns --input "$file" --output "results_${file}.json"
done
```

### DNS-over-HTTPS Issues

**Problem**: DoH queries are slow or failing

**Solution**:
```bash
# Try different provider
reverdns --dns-over-https \
  --doh-provider "https://cloudflare-dns.com/dns-query" \
  --input ips.txt

# Increase timeout for HTTPS
reverdns --dns-over-https --timeout 10 --input ips.txt
```

## 📝 FAQ

**Q: How many IPs can I process at once?**  
A: Theoretically unlimited. Practical limits depend on memory and network. Recommended: 100K-1M per batch.

**Q: Can I use private DNS servers?**  
A: Yes, use `--resolver` flag with your private server IP.

**Q: Is DNS-over-HTTPS slower?**  
A: Slightly, due to HTTPS overhead, but provides better privacy.

**Q: How do I handle rate limiting from DNS servers?**  
A: Adjust `--rate-limit` and `--concurrency`, or use multiple resolvers.

**Q: Can I use this in production?**  
A: Yes, it's designed for production use with proper error handling and monitoring.

**Q: What's the difference between JSON and CSV output?**  
A: JSON includes nested metadata; CSV is flat and easier to import into spreadsheets.

**Q: How do I monitor performance?**  
A: Use `--stats` flag to see statistics, or parse the JSON output.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Start for Contributors

```bash
# Clone and setup
git clone https://github.com/ismailtasdelen/ReverDNS.git
cd ReverDNS

# Create feature branch
git checkout -b feature/your-feature

# Make changes and test
cargo test
cargo fmt
cargo clippy

# Commit and push
git commit -m "feat: add your feature"
git push origin feature/your-feature

# Create Pull Request
```

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [trust-dns](https://github.com/bluejekyll/trust-dns) - DNS library
- [Tokio](https://tokio.rs/) - Async runtime
- [Clap](https://github.com/clap-rs/clap) - CLI framework
- [Serde](https://serde.rs/) - Serialization framework

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/ismailtasdelen/ReverDNS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ismailtasdelen/ReverDNS/discussions)
- **Documentation**: [README.md](README.md) and [QUICKSTART.md](QUICKSTART.md)

## 🗺️ Roadmap

### Version 2.1 (Planned)
- DNS caching layer
- Batch job scheduling
- Database backend support
- Advanced filtering and search

### Version 2.2 (Planned)
- Distributed processing
- Kubernetes operator
- GraphQL API
- Real-time streaming

### Version 3.0 (Future)
- Multi-protocol support (DNSSEC, DNS64)
- Advanced analytics
- Enterprise features
- Mobile app

---

**Version**: 2.0.0  
**Last Updated**: January 2024  
**Status**: Production Ready  
**Maintainer**: ReverDNS Team

For more information, visit the [GitHub Repository](https://github.com/ismailtasdelen/ReverDNS)
