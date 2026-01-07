# ReverDNS - High-Performance Reverse DNS Lookup Tool

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI/CD](https://github.com/ismailtasdelen/ReverDNS/workflows/CI/badge.svg)](https://github.com/ismailtasdelen/ReverDNS/actions)

A modern, high-performance CLI tool for bulk reverse DNS (PTR) lookups with support for JSON/CSV export, resolver rotation, rate limiting, DNS-over-HTTPS, and optional web API.

## 🎯 Features

- **High Performance**: Built in Rust with async/await using Tokio runtime
- **Bulk Processing**: Process thousands of IPs efficiently with concurrent lookups
- **Multiple Output Formats**: JSON and CSV export options
- **Resolver Rotation**: Automatic rotation between multiple DNS resolvers
- **Rate Limiting**: Built-in rate limiting to respect DNS server limits
- **DNS-over-HTTPS**: Secure DNS queries via HTTPS
- **Retry Logic**: Exponential backoff and automatic retry on failures
- **Comprehensive Metadata**: TTL, latency, error details in output
- **Web API** (Optional): REST API and web UI for programmatic access
- **Detailed Logging**: Structured logging with multiple output levels
- **Cross-Platform**: Works on Linux, macOS, and Windows

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
- [Configuration](#configuration)
- [Output Formats](#output-formats)
- [Examples](#examples)
- [Performance](#performance)
- [API Reference](#api-reference)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [License](#license)

## 🚀 Installation

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### From Source

```bash
git clone https://github.com/ismailtasdelen/ReverDNS.git
cd ReverDNS
cargo build --release
```

The binary will be available at `target/release/reverdns`.

### Using Cargo

```bash
cargo install reverdns
```

### Docker

```bash
docker build -t reverdns .
docker run --rm reverdns --help
```

### Binary Download

Download pre-compiled binaries from [Releases](https://github.com/ismailtasdelen/ReverDNS/releases).

## 🏃 Quick Start

### Basic Usage

```bash
# Single IP lookup
reverdns 8.8.8.8

# Batch processing from file
reverdns --input ips.txt --output results.json

# CSV output
reverdns --input ips.txt --output results.csv --format csv

# With custom resolver
reverdns --input ips.txt --resolver 1.1.1.1 --output results.json
```

## 📖 Usage

### Command-Line Options

```
USAGE:
    reverdns [OPTIONS] [IPS]...

ARGS:
    <IPS>...    IP addresses to lookup (can be provided as arguments or via --input)

OPTIONS:
    -i, --input <FILE>              Input file with IP addresses (one per line)
    -o, --output <FILE>             Output file path (default: stdout)
    -f, --format <FORMAT>           Output format: json, csv (default: json)
    -r, --resolver <RESOLVER>       Custom DNS resolver IP (can be used multiple times)
    -t, --timeout <SECONDS>         Timeout per lookup in seconds (default: 5)
    -c, --concurrency <NUM>         Number of concurrent lookups (default: 10)
    -l, --rate-limit <PER_SEC>      Rate limit: lookups per second (default: 100)
    --dns-over-https                Use DNS-over-HTTPS (DoH)
    --doh-provider <URL>            Custom DoH provider URL
    --retry-count <NUM>             Number of retries on failure (default: 3)
    --retry-backoff <MS>            Initial backoff in milliseconds (default: 100)
    --log-level <LEVEL>             Log level: trace, debug, info, warn, error (default: info)
    --stats                         Print statistics after completion
    --web-server                    Start web API server
    --web-port <PORT>               Web server port (default: 8080)
    -h, --help                      Print help information
    -V, --version                   Print version information
```

### Configuration File

Create a `.reverdns.toml` file in your project directory:

```toml
[dns]
timeout = 5
concurrency = 10
rate_limit = 100
retry_count = 3
retry_backoff_ms = 100

[resolvers]
# Use custom resolvers
custom = ["8.8.8.8", "1.1.1.1", "9.9.9.9"]

[output]
format = "json"
include_metadata = true

[logging]
level = "info"
format = "json"

[web]
enabled = false
port = 8080
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
reverdns --input ips.txt --output results.json --format json

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

### Example 5: Web API Server

```bash
# Start web server
reverdns --web-server --web-port 8080

# In another terminal, submit lookup job
curl -X POST http://localhost:8080/api/lookup \
  -H "Content-Type: application/json" \
  -d '{
    "ips": ["8.8.8.8", "1.1.1.1"],
    "format": "json"
  }'
```

### Example 6: CSV Export with Statistics

```bash
reverdns \
  --input ips.txt \
  --output results.csv \
  --format csv \
  --stats \
  --log-level info
```

## 📊 Performance

### Benchmarks

Performance metrics on modern hardware (Intel i7, 16GB RAM):

| Scenario | IPs | Concurrency | Time | Avg Latency |
|----------|-----|-------------|------|-------------|
| Small batch | 100 | 10 | 2.5s | 25ms |
| Medium batch | 1,000 | 25 | 8.3s | 28ms |
| Large batch | 10,000 | 50 | 45s | 32ms |
| Bulk processing | 100,000 | 100 | 380s | 35ms |

### Optimization Tips

1. **Increase Concurrency**: For network-bound operations, increase `--concurrency` (50-100 recommended)
2. **Adjust Rate Limiting**: Balance between speed and DNS server load
3. **Use Multiple Resolvers**: Distribute load across multiple DNS servers
4. **Enable DNS-over-HTTPS**: For better privacy and potentially faster responses
5. **Batch Processing**: Process large lists in chunks to manage memory

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

## 🛠️ Development

### Project Structure

```
ReverDNS/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── dns/
│   │   ├── mod.rs           # DNS module
│   │   ├── resolver.rs      # Resolver implementation
│   │   └── cache.rs         # DNS cache
│   ├── cli/
│   │   ├── mod.rs           # CLI module
│   │   └── args.rs          # Argument parsing
│   ├── output/
│   │   ├── mod.rs           # Output module
���   │   ├── json.rs          # JSON formatter
│   │   └── csv.rs           # CSV formatter
│   ├── web/
│   │   ├── mod.rs           # Web API module
│   │   ├── server.rs        # Web server
│   │   └── handlers.rs      # API handlers
│   ├── error.rs             # Error types
│   ├── config.rs            # Configuration
│   └── logger.rs            # Logging setup
├── tests/
│   ├── integration_tests.rs # Integration tests
│   └── fixtures/            # Test fixtures
├── benches/
│   └── dns_lookup_benchmark.rs
├── Cargo.toml               # Project manifest
├── Cargo.lock               # Dependency lock
├── README.md                # This file
├── CONTRIBUTING.md          # Contributing guidelines
├── LICENSE                  # MIT License
├── .github/
│   └── workflows/
│       └── ci.yml           # GitHub Actions CI/CD
└── .gitignore               # Git ignore rules
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# With all features
cargo build --release --all-features
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_dns_lookup

# Run integration tests
cargo test --test integration_tests

# Run with coverage
cargo tarpaulin --out Html
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Check for security issues
cargo audit

# Generate documentation
cargo doc --open
```

## 📝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Code Standards

- Follow Rust naming conventions and idioms
- Write tests for new features
- Ensure all tests pass: `cargo test`
- Run formatter: `cargo fmt`
- Run linter: `cargo clippy`
- Update documentation for API changes
- Use semantic commit messages

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes and add tests
4. Run tests and linting: `cargo test && cargo fmt && cargo clippy`
5. Commit with semantic messages: `git commit -m "feat: add new feature"`
6. Push to your fork: `git push origin feature/your-feature`
7. Create a Pull Request

## 🗺️ Roadmap

### Version 2.0 (Current)
- ✅ Rust rewrite with async/await
- ✅ JSON/CSV export
- ✅ Resolver rotation
- ✅ Rate limiting
- ✅ Comprehensive error handling
- ✅ Unit and integration tests
- ✅ CI/CD pipeline

### Version 2.1 (Planned)
- [ ] DNS caching layer
- [ ] Batch job scheduling
- [ ] Database backend support (PostgreSQL, SQLite)
- [ ] Advanced filtering and search
- [ ] Performance metrics dashboard

### Version 2.2 (Planned)
- [ ] Distributed processing support
- [ ] Kubernetes operator
- [ ] GraphQL API
- [ ] Real-time streaming results
- [ ] Machine learning-based anomaly detection

### Version 3.0 (Future)
- [ ] Multi-protocol support (DNSSEC, DNS64)
- [ ] Advanced analytics and reporting
- [ ] Enterprise features (RBAC, audit logging)
- [ ] Mobile app

## 📊 Statistics

- **Language**: Rust
- **Lines of Code**: ~5,000+
- **Test Coverage**: >85%
- **Dependencies**: 25+
- **Supported Platforms**: Linux, macOS, Windows

## 🐛 Known Issues

- DNS-over-HTTPS may have higher latency on first request (connection establishment)
- Very large batches (>1M IPs) may require memory optimization
- Some ISPs block DNS queries to non-standard resolvers

## ❓ FAQ

**Q: How many IPs can I process at once?**
A: Theoretically unlimited, but practical limits depend on available memory and network bandwidth. Recommended: process in batches of 100K-1M IPs.

**Q: Can I use this with private DNS servers?**
A: Yes, use the `--resolver` flag to specify your private DNS server IP.

**Q: Is DNS-over-HTTPS slower?**
A: Slightly, due to HTTPS overhead, but provides better privacy and security.

**Q: How do I handle rate limiting from DNS servers?**
A: Adjust `--rate-limit` and `--concurrency` parameters, or use multiple resolvers.

**Q: Can I use this in production?**
A: Yes, it's designed for production use. Ensure proper error handling and monitoring.

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [trust-dns](https://github.com/bluejekyll/trust-dns) - DNS library
- [Tokio](https://tokio.rs/) - Async runtime
- [Clap](https://github.com/clap-rs/clap) - CLI argument parsing
- Community contributors and testers

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/ismailtasdelen/ReverDNS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ismailtasdelen/ReverDNS/discussions)
- **Email**: support@reverdns.dev

---

**Last Updated**: January 2024  
**Maintainer**: ReverDNS Team
