# Contributing to ReverDNS

Thank you for your interest in contributing to ReverDNS! This document provides guidelines and instructions for contributing to the project.

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please read and adhere to our Code of Conduct:

- Be respectful and inclusive
- Welcome diverse perspectives
- Focus on constructive feedback
- Report inappropriate behavior to maintainers

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo
- Git
- Basic understanding of DNS and networking concepts

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/ismailtasdelen/ReverDNS.git
cd ReverDNS

# Install dependencies
cargo build

# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt
```

## Development Workflow

### 1. Create an Issue

Before starting work on a feature or bug fix:

1. Check existing issues to avoid duplicates
2. Create a new issue describing the problem or feature
3. Wait for feedback from maintainers
4. Get approval before starting significant work

### 2. Fork and Branch

```bash
# Fork the repository on GitHub

# Clone your fork
git clone https://github.com/YOUR_USERNAME/ReverDNS.git
cd ReverDNS

# Add upstream remote
git remote add upstream https://github.com/ismailtasdelen/ReverDNS.git

# Create a feature branch
git checkout -b feature/your-feature-name
```

### 3. Make Changes

Follow these guidelines:

- **One feature per branch**: Keep changes focused and manageable
- **Write tests**: Add tests for new functionality
- **Update documentation**: Update README and code comments as needed
- **Follow code style**: Use `cargo fmt` and `cargo clippy`

### 4. Commit Messages

Use semantic commit messages:

```
feat: add new feature
fix: fix a bug
docs: update documentation
style: format code
refactor: refactor code
perf: improve performance
test: add or update tests
chore: update dependencies
ci: update CI/CD configuration
```

Examples:
```
feat: add DNS-over-HTTPS support
fix: handle timeout errors gracefully
docs: update API documentation
test: add integration tests for resolver rotation
```

### 5. Testing

Ensure all tests pass before submitting a PR:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run integration tests
cargo test --test integration_tests

# Check code coverage
cargo tarpaulin --out Html
```

### 6. Code Quality

Before submitting a PR, ensure code quality:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Check for security issues
cargo audit

# Generate documentation
cargo doc --no-deps --open
```

### 7. Push and Create Pull Request

```bash
# Push your branch
git push origin feature/your-feature-name

# Create a Pull Request on GitHub
# - Provide a clear title and description
# - Reference related issues
# - Include screenshots/examples if applicable
```

## Pull Request Guidelines

### PR Title Format

Use semantic titles:
- `feat: add feature description`
- `fix: fix issue description`
- `docs: update documentation`
- `test: add tests for feature`

### PR Description Template

```markdown
## Description
Brief description of changes

## Related Issues
Closes #123

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added
- [ ] Integration tests added
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] No new warnings generated
- [ ] Tests pass locally
```

## Code Style Guide

### Rust Conventions

1. **Naming**:
   - Functions and variables: `snake_case`
   - Types and traits: `PascalCase`
   - Constants: `SCREAMING_SNAKE_CASE`

2. **Formatting**:
   - Use `cargo fmt` for automatic formatting
   - Line length: 100 characters (soft limit)
   - Use meaningful variable names

3. **Error Handling**:
   - Use `Result<T, E>` for fallible operations
   - Implement custom error types using `thiserror`
   - Provide context with error messages

4. **Documentation**:
   - Add doc comments to public items
   - Include examples in doc comments
   - Use `///` for item documentation
   - Use `//!` for module documentation

Example:
```rust
/// Performs a reverse DNS lookup for the given IP address.
///
/// # Arguments
/// * `ip` - The IP address to lookup
/// * `resolver` - The DNS resolver to use
///
/// # Returns
/// * `Ok(hostname)` - The resolved hostname
/// * `Err(error)` - If lookup fails
///
/// # Example
/// ```
/// let hostname = lookup_ip("8.8.8.8", resolver).await?;
/// println!("Hostname: {}", hostname);
/// ```
pub async fn lookup_ip(ip: &str, resolver: &Resolver) -> Result<String> {
    // Implementation
}
```

### Testing Standards

1. **Unit Tests**:
   - Place in same file as code
   - Use `#[cfg(test)]` module
   - Test both success and failure cases

2. **Integration Tests**:
   - Place in `tests/` directory
   - Test complete workflows
   - Use fixtures for test data

3. **Test Naming**:
   - `test_<function>_<scenario>`
   - Example: `test_lookup_ip_valid_address`

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lookup_ip_valid_address() {
        let resolver = create_test_resolver();
        let result = lookup_ip("8.8.8.8", &resolver).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_lookup_ip_invalid_address() {
        let resolver = create_test_resolver();
        let result = lookup_ip("invalid", &resolver).await;
        assert!(result.is_err());
    }
}
```

## Documentation

### README Updates

Update README.md if you:
- Add new features
- Change command-line arguments
- Modify output formats
- Add new examples

### Code Comments

- Add comments for complex logic
- Explain "why", not "what"
- Keep comments up-to-date with code

### API Documentation

- Document all public functions and types
- Include examples in doc comments
- Update API reference section in README

## Performance Considerations

When contributing, consider:

1. **Async/Await**: Use async for I/O operations
2. **Memory**: Avoid unnecessary allocations
3. **Concurrency**: Use Tokio for concurrent operations
4. **Benchmarking**: Run benchmarks for performance-critical code

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench dns_lookup
```

## Security

- Don't commit secrets or credentials
- Use `.gitignore` for sensitive files
- Report security issues privately to maintainers
- Follow Rust security best practices

## Dependency Management

### Adding Dependencies

1. Evaluate necessity and maintenance status
2. Check for security issues: `cargo audit`
3. Prefer well-maintained crates
4. Update Cargo.toml and Cargo.lock

```bash
# Add dependency
cargo add dependency_name

# Update dependencies
cargo update

# Check for security issues
cargo audit
```

### Removing Dependencies

- Remove unused dependencies
- Update Cargo.toml
- Run `cargo test` to ensure nothing breaks

## Release Process

Maintainers follow semantic versioning (SemVer):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

Release checklist:
1. Update version in Cargo.toml
2. Update CHANGELOG.md
3. Create git tag: `git tag v2.0.0`
4. Push tag: `git push origin v2.0.0`
5. GitHub Actions automatically builds and publishes

## Getting Help

- **Questions**: Open a GitHub Discussion
- **Issues**: Check existing issues first
- **Documentation**: Read README and code comments
- **Community**: Join our Discord/Slack (if available)

## Recognition

Contributors will be:
- Added to CONTRIBUTORS.md
- Mentioned in release notes
- Recognized in project documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to:
- Open an issue with the `question` label
- Start a GitHub Discussion
- Contact maintainers directly

Thank you for contributing to ReverDNS! 🎉
