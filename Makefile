.PHONY: all build test clean install dev-setup help

# Default target
all: build

# Build the compiler in release mode
build:
	@echo "🔨 Building Fera compiler..."
	cargo build --release
	@echo "✅ Build complete: target/release/fera"

# Build in development mode
dev:
	@echo "🔨 Building Fera compiler (debug)..."
	cargo build
	@echo "✅ Build complete: target/debug/fera"

# Run tests
test:
	@echo "🧪 Running tests..."
	cargo test

# Run clippy for linting
lint:
	@echo "🔍 Running clippy..."
	cargo clippy -- -D warnings

# Format code
fmt:
	@echo "✨ Formatting code..."
	cargo fmt

# Check code without building
check:
	@echo "🔍 Checking code..."
	cargo check

# Clean build artifacts
clean:
	@echo "🧹 Cleaning..."
	cargo clean
	rm -rf build/
	@echo "✅ Clean complete"

# Install to system
install: build
	@echo "📦 Installing fera..."
	cargo install --path .
	@echo "✅ Installed to ~/.cargo/bin/fera"

# Development setup
dev-setup:
	@echo "🔧 Setting up development environment..."
	@command -v rustup >/dev/null 2>&1 || { echo "Installing rustup..."; curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; }
	rustup update
	rustup component add rustfmt clippy
	@echo "✅ Development environment ready"

# Run examples
example-hello: build
	@echo "Running hello world example..."
	target/release/fera run examples/hello.fera

# Quick validation
validate: fmt lint test
	@echo "✅ All checks passed"

# Help
help:
	@echo "Fera Makefile Commands:"
	@echo "  make build       - Build compiler in release mode"
	@echo "  make dev         - Build compiler in debug mode"
	@echo "  make test        - Run all tests"
	@echo "  make lint        - Run clippy linter"
	@echo "  make fmt         - Format source code"
	@echo "  make check       - Type-check without building"
	@echo "  make clean       - Remove build artifacts"
	@echo "  make install     - Install to ~/.cargo/bin"
	@echo "  make dev-setup   - Setup development environment"
	@echo "  make validate    - Run fmt, lint, and tests"
	@echo "  make help        - Show this help message"

