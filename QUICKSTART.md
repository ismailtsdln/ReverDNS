# Quick Start Guide

Get up and running with ReverDNS in minutes!

## Installation

### Option 1: Build from Source

```bash
git clone https://github.com/ismailtasdelen/ReverDNS.git
cd ReverDNS
cargo build --release
./target/release/reverdns --version
```

### Option 2: Using Cargo

```bash
cargo install reverdns
reverdns --version
```

### Option 3: Docker

```bash
docker build -t reverdns .
docker run --rm reverdns --version
```

## Basic Usage

### Single IP Lookup

```bash
reverdns 8.8.8.8
```

### Multiple IPs

```bash
reverdns 8.8.8.8 1.1.1.1 9.9.9.9
```

### From File

Create `ips.txt`:
```
8.8.8.8
1.1.1.1
9.9.9.9
```

Then run:
```bash
reverdns --input ips.txt --output results.json
```

### CSV Output

```bash
reverdns --input ips.txt --output results.csv --format csv
```

## Common Examples

### High-Performance Batch Processing

```bash
reverdns \
  --input large_list.txt \
  --output results.json \
  --concurrency 50 \
  --rate-limit 500 \
  --timeout 10 \
  --stats
```

### With Custom Resolver

```bash
reverdns \
  --input ips.txt \
  --output results.json \
  --resolver 8.8.8.8
```

### Multiple Resolvers

```bash
reverdns \
  --input ips.txt \
  --output results.json \
  --resolver 8.8.8.8 \
  --resolver 1.1.1.1 \
  --resolver 9.9.9.9
```

### DNS-over-HTTPS

```bash
reverdns \
  --input ips.txt \
  --output results.json \
  --dns-over-https \
  --doh-provider "https://dns.google/dns-query"
```

### With Detailed Logging

```bash
reverdns \
  --input ips.txt \
  --output results.json \
  --log-level debug \
  --stats
```

## Output Examples

### JSON Output

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
    }
  ],
  "metadata": {
    "total_lookups": 1,
    "successful": 1,
    "failed": 0,
    "total_time_ms": 100,
    "average_latency_ms": 45.0
  }
}
```

### CSV Output

```csv
ip,hostname,status,ttl,latency_ms,resolver,error,timestamp
8.8.8.8,dns.google,success,3600,45,8.8.8.8,,2024-01-15T10:30:45Z
```

## Tips & Tricks

### Process Large Files

For very large IP lists, process in batches:

```bash
# Split file into chunks
split -l 10000 large_list.txt chunk_

# Process each chunk
for file in chunk_*; do
  reverdns --input "$file" --output "results_${file}.json"
done
```

### Monitor Progress

Use `--log-level debug` to see detailed progress:

```bash
reverdns --input ips.txt --output results.json --log-level debug
```

### Optimize for Speed

```bash
reverdns \
  --input ips.txt \
  --output results.json \
  --concurrency 100 \
  --rate-limit 1000 \
  --timeout 3
```

### Optimize for Reliability

```bash
reverdns \
  --input ips.txt \
  --output results.json \
  --concurrency 10 \
  --rate-limit 50 \
  --timeout 10 \
  --retry-count 5
```

## Troubleshooting

### "Connection refused" Error

- Check your DNS resolver is accessible
- Try using a public resolver: `--resolver 8.8.8.8`

### Slow Performance

- Increase concurrency: `--concurrency 50`
- Increase rate limit: `--rate-limit 500`
- Use multiple resolvers

### High Failure Rate

- Increase timeout: `--timeout 10`
- Increase retry count: `--retry-count 5`
- Check your network connection

### Out of Memory

- Reduce concurrency: `--concurrency 5`
- Process in smaller batches
- Use CSV output instead of JSON

## Next Steps

- Read the [README.md](README.md) for comprehensive documentation
- Check [CONTRIBUTING.md](CONTRIBUTING.md) to contribute
- See [DEVELOPMENT.md](DEVELOPMENT.md) for development setup
- Review [examples](examples/) for more use cases

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/ismailtasdelen/ReverDNS/issues)
- **Discussions**: [GitHub Discussions](https://github.com/ismailtasdelen/ReverDNS/discussions)
- **Documentation**: [README.md](README.md)

---

Happy DNS lookups! 🚀
