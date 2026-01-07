.PHONY: help build release test fmt lint audit clean doc bench install

help:
	@echo "ReverDNS - Development Commands"
	@echo "================================"
	@echo "make build       - Build debug binary"
	@echo "make release     - Build optimized release binary"
	@echo "make test        - Run all tests"
	@echo "make fmt         - Format code"
	@echo "make lint        - Run clippy linter"
	@echo "make audit       - Check for security vulnerabilities"
	@echo "make clean       - Clean build artifacts"
	@echo "make doc         - Generate and open documentation"
	@echo "make bench       - Run benchmarks"
	@echo "make install     - Install binary locally"
	@echo "make docker      - Build Docker image"
	@echo "make docker-run  - Run Docker container"

build:
	cargo build

release:
	cargo build --release

test:
	cargo test --verbose

test-integration:
	cargo test --test integration_tests --verbose

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

audit:
	cargo audit

clean:
	cargo clean

doc:
	cargo doc --no-deps --open

bench:
	cargo bench

install:
	cargo install --path .

docker:
	docker build -t reverdns:latest .

docker-run:
	docker run --rm reverdns:latest --help

docker-compose-up:
	docker-compose up -d

docker-compose-down:
	docker-compose down

check: fmt lint test audit
	@echo "All checks passed!"

all: clean build test lint audit doc
	@echo "Build complete!"
