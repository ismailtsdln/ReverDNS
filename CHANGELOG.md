# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2024-01-15

### Added
- Complete rewrite in Rust with async/await support
- High-performance DNS lookup with Tokio runtime
- JSON and CSV output formats
- Resolver rotation support
- Rate limiting functionality
- DNS-over-HTTPS (DoH) support
- Comprehensive error handling with custom error types
- Retry logic with exponential backoff
- Detailed metadata in output (TTL, latency, timestamps)
- Unit and integration test suite
- CI/CD pipeline with GitHub Actions
- Performance benchmarks
- Web API server (optional)
- Structured logging with multiple output levels
- Cross-platform support (Linux, macOS, Windows)
- Comprehensive documentation and examples

### Changed
- Migrated from Go to Rust for better performance and safety
- Improved CLI argument parsing with Clap
- Enhanced error messages and logging

### Fixed
- Better timeout handling
- Improved network error recovery
- More reliable DNS resolution

## [1.0.0] - 2023-01-01

### Added
- Initial Go-based implementation
- Basic reverse DNS lookup functionality
- JSON output support
- CLI interface

---

## Unreleased

### Planned for 2.1
- DNS caching layer
- Batch job scheduling
- Database backend support (PostgreSQL, SQLite)
- Advanced filtering and search
- Performance metrics dashboard

### Planned for 2.2
- Distributed processing support
- Kubernetes operator
- GraphQL API
- Real-time streaming results
- Machine learning-based anomaly detection

### Planned for 3.0
- Multi-protocol support (DNSSEC, DNS64)
- Advanced analytics and reporting
- Enterprise features (RBAC, audit logging)
- Mobile app
