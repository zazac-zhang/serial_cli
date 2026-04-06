#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
SHA256="${2:-}"

if [ -z "$VERSION" ] || [ -z "$SHA256" ]; then
    echo "Usage: update_scoop.sh <version> <sha256>"
    exit 1
fi

BUCKET_REPO="https://github.com/zazac-zhang/serial-cli-scoop.git"
TMP_DIR=$(mktemp -d)

echo "Cloning Scoop bucket repository..."
git clone "$BUCKET_REPO" "$TMP_DIR"
cd "$TMP_DIR"

MANIFEST_FILE="bucket/serial-cli.json"

# Remove version prefix 'v' for Scoop
VERSION_NUM="${VERSION#v}"

if [ -f "$MANIFEST_FILE" ]; then
    # Update existing manifest
    cat > "$MANIFEST_FILE" << EOF
{
  "version": "$VERSION_NUM",
  "description": "Universal serial port CLI tool",
  "homepage": "https://github.com/zazac-zhang/serial_cli",
  "license": "MIT OR Apache-2.0",
  "url": "https://github.com/zazac-zhang/serial_cli/releases/download/${VERSION}/serial-cli-windows-x86_64.exe",
  "hash": "$SHA256",
  "bin": "serial-cli.exe",
  "post_install": [
    "Write-Host \"Serial CLI $VERSION_NUM installed successfully!\""
  ]
}
EOF
    echo "✓ Updated existing manifest"
else
    # Create new manifest
    mkdir -p bucket
    cat > "$MANIFEST_FILE" << EOF
{
  "version": "$VERSION_NUM",
  "description": "Universal serial port CLI tool",
  "homepage": "https://github.com/zazac-zhang/serial_cli",
  "license": "MIT OR Apache-2.0",
  "url": "https://github.com/zazac-zhang/serial_cli/releases/download/${VERSION}/serial-cli-windows-x86_64.exe",
  "hash": "$SHA256",
  "bin": "serial-cli.exe",
  "post_install": [
    "Write-Host \"Serial CLI $VERSION_NUM installed successfully!\""
  ]
}
EOF
    echo "✓ Created new manifest"
fi

# Commit and push
git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"
git add "$MANIFEST_FILE"
git commit -m "Bump to $VERSION_NUM"
git push

echo "✓ Scoop bucket updated"

# Cleanup
rm -rf "$TMP_DIR"
