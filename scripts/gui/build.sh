#!/usr/bin/env bash
set -euo pipefail

TARGET="${1:-}"

echo "Building Serial CLI GUI..."

# Build frontend
echo "Building frontend..."
cd src-ui
npm install
npm run build
cd ..

# Build Tauri application
echo "Building Tauri application..."
cd src-tauri
if [ -n "$TARGET" ]; then
    cargo tauri build --target "$TARGET"
else
    cargo tauri build
fi
cd ..

echo ""
echo "✓ Build complete"
echo "Artifacts: src-tauri/target/release/bundle/"
