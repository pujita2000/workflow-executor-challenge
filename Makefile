# Workflow Executor Challenge Makefile

.PHONY: help build test test-basic test-advanced test-all clean package run-examples lint fmt check install dev-setup

# Default target
help:
	@echo "Workflow Executor Challenge - Available Commands:"
	@echo ""
	@echo "  make build          - Build the project in debug mode"
	@echo "  make build-release  - Build the project in release mode"
	@echo "  make test           - Run all tests"
	@echo "  make test-basic     - Run basic tests only (required)"
	@echo "  make test-advanced  - Run advanced tests only (bonus)"
	@echo "  make test-verbose   - Run all tests with output"
	@echo "  make check          - Run cargo check"
	@echo "  make lint           - Run clippy for linting"
	@echo "  make fmt            - Format code with rustfmt"
	@echo "  make fmt-check      - Check code formatting"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make package        - Package the challenge for distribution"
	@echo "  make dev-setup      - Install development dependencies"
	@echo ""

# Build targets
build:
	@echo "Building workflow executor..."
	cargo build

build-release:
	@echo "Building workflow executor (release)..."
	cargo build --release

# Test targets
test:
	@echo "Running all tests..."
	cargo test

test-basic:
	@echo "Running basic tests (required)..."
	cargo test --test basic_tests

test-advanced:
	@echo "Running advanced tests (bonus)..."
	cargo test --test advanced_tests

test-all: test-basic test-advanced

test-verbose:
	@echo "Running all tests with output..."
	cargo test -- --nocapture

test-output:
	@echo "Running tests and saving output..."
	cargo test --test basic_tests > test_output.txt 2>&1
	@echo "Test output saved to test_output.txt"

# Code quality targets
check:
	@echo "Running cargo check..."
	cargo check

lint:
	@echo "Running clippy..."
	cargo clippy -- -D warnings

fmt:
	@echo "Formatting code..."
	cargo fmt

fmt-check:
	@echo "Checking code formatting..."
	cargo fmt -- --check

# Clean targets
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf dist/challenge
	rm -f dist/workflow-executor-challenge.zip
	rm -f test_output.txt

clean-all: clean
	@echo "Cleaning all generated files..."
	rm -rf dist

# Package the challenge for distribution
package:
	@echo "Packaging challenge..."
	./scripts/package_challenge.sh

# Development setup
dev-setup:
	@echo "Installing development dependencies..."
	rustup component add clippy rustfmt

# Watch for changes and run tests
watch-test:
	@echo "Watching for changes and running tests..."
	@which cargo-watch > /dev/null || (echo "Installing cargo-watch..." && cargo install cargo-watch)
	cargo watch -x test

watch-test-basic:
	@echo "Watching for changes and running basic tests..."
	@which cargo-watch > /dev/null || (echo "Installing cargo-watch..." && cargo install cargo-watch)
	cargo watch -x "test --test basic_tests"

# CI/CD style checks
ci: fmt-check lint test-basic
	@echo "All CI checks passed!"

# Full verification before submission
verify: clean build test-basic test-advanced lint fmt-check
	@echo "Full verification complete!"
	@echo "Generating test output..."
	@$(MAKE) test-output
	@echo ""
	@echo "Submission checklist:"
	@echo "  ✓ Code builds successfully"
	@echo "  ✓ All basic tests pass"
	@echo "  ✓ Code is formatted"
	@echo "  ✓ No lint warnings"
	@echo ""
	@echo "Ready to submit! Check test_output.txt for test results."

# Quick development cycle
dev: fmt build test-basic

# Full test with timing information
test-timing:
	@echo "Running tests with timing..."
	cargo test -- --nocapture --test-threads=1

# Documentation
doc:
	@echo "Building documentation..."
	cargo doc --no-deps --open

doc-all:
	@echo "Building documentation with dependencies..."
	cargo doc --open

# Install/uninstall (if it were a binary)
install:
	@echo "Installing workflow executor..."
	cargo install --path .

# Benchmark (if you add benches)
bench:
	@echo "Running benchmarks..."
	cargo bench

# Show project info
info:
	@echo "Project: profound-workflow-executor-challenge"
	@echo "Rust version:"
	@rustc --version
	@echo "Cargo version:"
	@cargo --version
	@echo ""
	@echo "Dependencies:"
	@cargo tree --depth 1
