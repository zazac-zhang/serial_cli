#!/usr/bin/env bash
set -euo pipefail

CLI_VERSION="${1:-}"
GUI_INCREMENT="${2:-patch}"

if [ -z "$CLI_VERSION" ]; then
    echo "Usage: prepare-release.sh <cli_version> [gui_increment]"
    exit 1
fi

echo "Preparing GUI release..."
echo "  CLI base version: $CLI_VERSION"
echo "  GUI increment: $GUI_INCREMENT"

# Update version
scripts/gui/update-version.sh "$CLI_VERSION" "$GUI_INCREMENT"

# Build GUI
scripts/gui/build.sh

echo ""
echo "✓ GUI release preparation complete"
echo "Review artifacts in: src-tauri/target/release/bundle/"
