#!/usr/bin/env bash
set -euo pipefail

TARGET="${1:-}"

echo "Building Serial CLI..."

if [ -n "$TARGET" ]; then
    echo "Building for target: $TARGET"
    cargo build --release --target "$TARGET"
else
    echo "Building for native target"
    cargo build --release
fi

echo "✓ Build complete"
echo "Binary: target/release/serial-cli"
