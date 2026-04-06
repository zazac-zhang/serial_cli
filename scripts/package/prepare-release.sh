#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"

if [ -z "$VERSION" ]; then
    echo "Usage: prepare-release.sh <version>"
    echo "Example: prepare-release.sh v1.2.3"
    exit 1
fi

echo "Preparing release $VERSION..."

# Remove 'v' prefix if present
VERSION_NUM="${VERSION#v}"

# Update Cargo.toml
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$VERSION_NUM\"/" Cargo.toml

echo "Updated Cargo.toml version: $CURRENT_VERSION -> $VERSION_NUM"

# Generate changelog
if command -v git-cliff &> /dev/null; then
    git-cliff --tag "$VERSION" --output CHANGELOG.md
    echo "Generated CHANGELOG.md"
else
    echo "⚠ git-cliff not installed, skipping changelog generation"
fi

echo "✓ Release preparation complete"
echo "Review changes and commit with: git commit -am \"chore: prepare release $VERSION\""
