# Development Guide

This guide provides information for developers working on the ReverDNS project.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Testing](#testing)
- [Performance Optimization](#performance-optimization)
- [Debugging](#debugging)
- [Common Tasks](#common-tasks)

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo
- Git
- Docker (optional, for containerized development)

### Initial Setup

```bash
# Clone the repository
git clone https://github.com/ismailtasdelen/ReverDNS.git
cd ReverDNS

# Install dependencies
cargo build

# Run tests
cargo test

# Run the application
cargo run -- --help
```

## Project Structure

```
src/
├── main.rs              # CLI entry point
├── lib.rs               # Library root
├── cli/
│   ├── mod.rs           # CLI module
│   └── args.rs          # Argument parsing
├── dns/
│   ├── mod.rs           # DNS module
│   └── resolver.rs      # DNS resolver implementation
├── output/
��   ├── mod.rs           # Output module
│   ├── json.rs          # JSON formatter
│   └── csv.rs           # CSV formatter
├── error.rs             # Error types
└── logger.rs            # Logging setup

tests/
└── integration_tests.rs # Integration tests

benches/
└── dns_lookup_benchmark.rs # Performance benchmarks
```

## Development Workflow

### 1. Create a Feature Branch

```bash
git checkout -b feature/your-feature-name
```

### 2. Make Changes

Follow the code style guidelines:
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write tests for new features
- Update documentation

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration_tests
```

### 4. Commit and Push

```bash
git add .
git commit -m "feat: add new feature"
git push origin feature/your-feature-name
```

### 5. Create a Pull Request

Create a PR on GitHub with a clear description of your changes.

## Testing

### Unit Tests

Unit tests are located in the same file as the code they test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
    }
}
```

### Integration Tests

Integration tests are in the `tests/` directory:

```bash
cargo test --test integration_tests
```

### Running Tests with Coverage

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Performance Optimization

### Profiling

```bash
# Run with profiling
CARGO_PROFILE_RELEASE_DEBUG=true cargo build --release
perf record -g ./target/release/reverdns
perf report
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench dns_lookup
```

### Memory Profiling

```bash
# Using valgrind (Linux)
valgrind --leak-check=full ./target/release/reverdns
```

## Debugging

### Using RUST_LOG

```bash
# Set log level
RUST_LOG=debug cargo run -- --help

# Specific module
RUST_LOG=reverdns::dns=debug cargo run -- --help
```

### Using a Debugger

```bash
# With lldb (macOS)
lldb ./target/debug/reverdns

# With gdb (Linux)
gdb ./target/debug/reverdns
```

### Debugging in VS Code

Create `.vscode/launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug",
            "cargo": {
                "args": [
                    "build",
                    "--bin=reverdns",
                    "--package=reverdns"
                ],
                "filter": {
                    "name": "reverdns",
                    "kind": "bin"
                }
            },
            "args": ["--help"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

## Common Tasks

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Build for specific target
cargo build --target x86_64-unknown-linux-gnu
```

### Formatting and Linting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy

# Run linter with warnings as errors
cargo clippy -- -D warnings
```

### Documentation

```bash
# Generate documentation
cargo doc

# Generate and open in browser
cargo doc --open

# Generate for dependencies too
cargo doc --no-deps --open
```

### Dependency Management

```bash
# Add dependency
cargo add dependency_name

# Update dependencies
cargo update

# Check for security issues
cargo audit

# Outdated dependencies
cargo outdated
```

### Docker Development

```bash
# Build Docker image
docker build -t reverdns:dev .

# Run container
docker run --rm reverdns:dev --help

# Run with volume mount
docker run --rm -v $(pwd)/data:/data reverdns:dev --input /data/ips.txt
```

## Code Style Guidelines

### Naming Conventions

- Functions and variables: `snake_case`
- Types and traits: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`

### Error Handling

Always use `Result<T>` for fallible operations:

```rust
pub fn do_something() -> Result<String> {
    // Implementation
}
```

### Documentation

Add doc comments to public items:

```rust
/// Brief description
///
/// Longer description if needed
///
/// # Arguments
/// * `arg1` - Description
///
/// # Returns
/// Description of return value
///
/// # Errors
/// Description of possible errors
///
/// # Example
/// ```
/// let result = do_something()?;
/// ```
pub fn do_something() -> Result<String> {
    // Implementation
}
```

## Troubleshooting

### Build Issues

```bash
# Clean build
cargo clean
cargo build

# Update dependencies
cargo update
```

### Test Failures

```bash
# Run with backtrace
RUST_BACKTRACE=1 cargo test

# Run with output
cargo test -- --nocapture
```

### Performance Issues

1. Profile the code using `cargo flamegraph`
2. Check for unnecessary allocations
3. Use async/await for I/O operations
4. Consider using `--release` build

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Tokio Documentation](https://tokio.rs/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)

## Getting Help

- Check existing issues and discussions
- Read the documentation
- Ask in Rust community forums
- Contact maintainers

---

Happy coding! 🦀
