run:
	RUSTFLAGS="-A warnings" cargo run

# Run with warnings visible
run-warnings:
	cargo run

# Check code without running
check:
	cargo check

# Run tests
test:
	cargo test

# Build release version
build-release:
	cargo build --release

# Run clippy for code quality
clippy:
	cargo clippy

# Format code
fmt:
	cargo fmt

# Clean build artifacts
clean:
	cargo clean

# Show help
help:
	@echo "Available targets:"
	@echo "  run          - Run with warnings suppressed"
	@echo "  run-warnings - Run with warnings visible"
	@echo "  check        - Check code without running"
	@echo "  test         - Run tests"
	@echo "  build-release- Build optimized release version"
	@echo "  clippy       - Run clippy linter"
	@echo "  fmt          - Format code"
	@echo "  clean        - Clean build artifacts"
	@echo "  help         - Show this help"

.PHONY: run run-warnings check test build-release clippy fmt clean help