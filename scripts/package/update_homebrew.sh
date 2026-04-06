#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
SHA256="${2:-}"

if [ -z "$VERSION" ] || [ -z "$SHA256" ]; then
    echo "Usage: update_homebrew.sh <version> <sha256>"
    echo "Example: update_homebrew.sh v1.2.3 abc123..."
    exit 1
fi

TAP_REPO="https://github.com/zazac-zhang/serial-cli-homebrew.git"
TMP_DIR=$(mktemp -d)

echo "Cloning Homebrew tap repository..."
git clone "$TAP_REPO" "$TMP_DIR"
cd "$TMP_DIR"

# Update formula
FORMULA_FILE="Formula/serial-cli.rb"
if [ -f "$FORMULA_FILE" ]; then
    # Update version and sha256
    sed -i.bak "s/v{version}/$VERSION/g" "$FORMULA_FILE"
    sed -i.bak "s/{sha256_from_release}/$SHA256/g" "$FORMULA_FILE"
    rm -f "$FORMULA_FILE.bak"
    echo "✓ Updated existing formula"
else
    # Create new formula
    cat > "$FORMULA_FILE" << EOF
class SerialCli < Formula
  desc "Universal serial port CLI tool optimized for AI interaction"
  homepage "https://github.com/zazac-zhang/serial_cli"
  url "https://github.com/zazac-zhang/serial_cli/archive/refs/tags/${VERSION}.tar.gz"
  sha256 "$SHA256"
  license any_of: ["MIT", "Apache-2.0"]

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--path", "."
  end

  test do
    system "#{bin}/serial-cli", "--version"
  end
end
EOF
    echo "✓ Created new formula"
fi

# Commit and push
git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"
git add "$FORMULA_FILE"
git commit -m "Bump to $VERSION"
git push

echo "✓ Homebrew tap updated"

# Cleanup
rm -rf "$TMP_DIR"
