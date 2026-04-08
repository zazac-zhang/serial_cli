# Serial CLI - Just Command Configuration
# https://github.com/casey/just

# Default list (run `just` to show)
default:
    @just --list

# =============================================================================
# Development Commands
# =============================================================================

# Development build (unoptimized)
dev:
    cargo build

# Release build (optimized)
build:
    cargo build --release

# Run application (development mode)
run *args:
    cargo run -- {{args}}

# =============================================================================
# Testing Commands
# =============================================================================

# Run all tests
test:
    cargo test

# Run tests with verbose output
test-verbose:
    cargo test -- --nocapture

# Run tests on file changes
test-watch:
    cargo watch -x test

# Run specific test
test-test name *args:
    cargo test {{name}} -- {{args}}

# =============================================================================
# Code Quality
# =============================================================================

# Run Clippy linter
lint:
    cargo clippy -- -D warnings

# Format code
fmt:
    cargo fmt

# Check code formatting
fmt-check:
    cargo fmt -- --check

# Run all checks (format + lint + test)
check: fmt-check lint test

# =============================================================================
# Clean Commands
# =============================================================================

# Clean build artifacts
clean:
    cargo clean

# Clean all (including target/)
clean-all:
    rm -rf target/

# =============================================================================
# Documentation Commands
# =============================================================================

# Generate and open documentation
docs:
    cargo doc --open

# Generate documentation
docs-build:
    cargo doc

# =============================================================================
# Cross-Compilation
# =============================================================================

# Build for all platforms
build-all: build-linux build-macos build-windows
    @echo "✓ All platforms built successfully"

# Build for Linux (x86_64 + aarch64)
build-linux:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building Linux x86_64..."
    cargo build --release --target x86_64-unknown-linux-gnu
    echo "Building Linux aarch64..."
    if command -v cross &> /dev/null; then
        cross build --release --target aarch64-unknown-linux-gnu
    else
        echo "⚠ Warning: 'cross' not installed. Install with: cargo install cross"
        echo "Skipping aarch64 build"
    fi

# Build for macOS (x86_64 + arm64)
build-macos:
    #!/usr/bin/env bash
    set -euo pipefail
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Building macOS x86_64..."
        cargo build --release --target x86_64-apple-darwin
        echo "Building macOS arm64..."
        cargo build --release --target aarch64-apple-darwin
    else
        echo "⚠ macOS builds can only be performed on macOS"
        echo "Skipping macOS build"
    fi

# Build for Windows (requires cross)
build-windows:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building Windows x86_64..."
    if command -v cross &> /dev/null; then
        cross build --release --target x86_64-pc-windows-msvc
    else
        echo "⚠ Warning: 'cross' not installed. Install with: cargo install cross"
        echo "Skipping Windows build"
    fi

# Full release build (clean + all platforms)
release: clean-all build-all
    @echo "✓ Release builds complete"

# =============================================================================
# GUI Commands
# =============================================================================

# Install GUI dependencies
gui-deps:
    cd frontend && npm install

# Start frontend development server
gui-dev-frontend:
    cd frontend && npm run dev

# Start Tauri GUI development
gui-dev:
    cargo tauri dev

# Build GUI application
gui-build:
    cd frontend && npm run build
    cargo tauri build

# Build frontend only
gui-build-frontend:
    cd frontend && npm run build

# Type check frontend
gui-type-check:
    cd frontend && npm run type-check

# Run frontend tests
gui-test:
    cd frontend && npm test

# Clean GUI artifacts
gui-clean:
    rm -rf frontend/dist
    rm -rf frontend/node_modules
    rm -rf src-tauri/target

# Check GUI Rust code
gui-check:
    cargo check --workspace

# Format GUI code
gui-fmt:
    cargo fmt
    cd frontend && npx prettier --write "src/**/*.{ts,tsx,css}"

# =============================================================================
# Installation
# =============================================================================

# Install locally (development version)
install:
    cargo install --path .

# Install locally (Release version)
install-release: build
    cargo install --path .
