#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
SHA256="${2:-}"

if [ -z "$VERSION" ] || [ -z "$SHA256" ]; then
    echo "Usage: generate_aur.sh <version> <sha256>"
    exit 1
fi

# Remove version prefix
VERSION_NUM="${VERSION#v}"

OUTPUT_DIR="aur"
mkdir -p "$OUTPUT_DIR"

echo "Generating AUR files for version $VERSION..."

# Generate PKGBUILD
cat > "$OUTPUT_DIR/PKGBUILD" << EOF
pkgname=serial-cli
pkgver=$VERSION_NUM
pkgrel=1
pkgdesc="Universal serial port CLI tool optimized for AI interaction"
arch=('x86_64' 'aarch64')
url="https://github.com/zazac-zhang/serial_cli"
license=('MIT' 'Apache')
depends=('gcc-libs')
makedepends=('cargo')
source=("\$pkgname-\$pkgver.tar.gz::https://github.com/zazac-zhang/serial_cli/archive/refs/tags/v\$pkgver.tar.gz")
sha256sums=('$SHA256')

build() {
  cd "\$pkgname-\$pkgver"
  cargo build --release
}

package() {
  cd "\$pkgname-\$pkgver"
  install -Dm755 "target/release/serial-cli" "\$pkgdir/usr/bin/serial-cli"
}
EOF

# Generate .SRCINFO
cat > "$OUTPUT_DIR/.SRCINFO" << EOF
pkgbase = serial-cli
pkgdesc = Universal serial port CLI tool optimized for AI interaction
pkgver = $VERSION_NUM
pkgrel = 1
url = https://github.com/zazac-zhang/serial_cli
arch = x86_64
arch = aarch64
license = MIT
license = Apache
depends = gcc-libs
makedepends = cargo
source = serial-cli-$VERSION_NUM.tar.gz::https://github.com/zazac-zhang/serial_cli/archive/refs/tags/v$VERSION_NUM.tar.gz
sha256sums = $SHA256
EOF

echo "✓ Generated AUR files:"
echo "  - $OUTPUT_DIR/PKGBUILD"
echo "  - $OUTPUT_DIR/.SRCINFO"
echo ""
echo "To submit to AUR:"
echo "  1. cd $OUTPUT_DIR"
echo "  2. git clone ssh://aur@aur.archlinux.org/serial-cli.git"
echo "  3. cp PKGBUILD .SRCINFO serial-cli/"
echo "  4. cd serial-cli && makepkg --printsrcinfo > .SRCINFO"
echo "  5. git add PKGBUILD .SRCINFO && git commit -m 'Update to $VERSION'"
echo "  6. git push"
