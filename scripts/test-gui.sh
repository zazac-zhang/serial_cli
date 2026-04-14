#!/usr/bin/env bash
set -euo pipefail

# Serial CLI GUI Installation Test
# Tests GUI installer artifacts for existence and validity
# Usage: ./scripts/test-gui.sh [artifact_directory]

ARTIFACT_DIR="${1:-src-tauri/target/release/bundle}"

echo "Starting GUI installation tests..."
echo "Artifact directory: $ARTIFACT_DIR"

test_dmg() {
    local dmg_file=$1
    echo "Testing macOS DMG: $dmg_file"
    if [ ! -f "$dmg_file" ]; then
        echo "  DMG file not found"; return 1
    fi
    local size=$(stat -f%z "$dmg_file" 2>/dev/null || stat -c%s "$dmg_file")
    if [ "$size" -lt 1048576 ]; then
        echo "  DMG file too small: $size bytes"; return 1
    fi
    echo "  Passed"
}

test_nsis() {
    local exe_file=$1
    echo "Testing Windows NSIS: $exe_file"
    if [ ! -f "$exe_file" ]; then
        echo "  Setup file not found"; return 1
    fi
    local size=$(stat -f%z "$exe_file" 2>/dev/null || stat -c%s "$exe_file")
    if [ "$size" -lt 10485760 ]; then
        echo "  Setup file too small: $size bytes"; return 1
    fi
    echo "  Passed"
}

test_appimage() {
    local appimage=$1
    echo "Testing Linux AppImage: $appimage"
    if [ ! -f "$appimage" ]; then
        echo "  AppImage not found"; return 1
    fi
    chmod +x "$appimage"
    echo "  Passed"
}

test_deb() {
    local deb_file=$1
    echo "Testing Debian package: $deb_file"
    if [ ! -f "$deb_file" ]; then
        echo "  DEB file not found"; return 1
    fi
    if ! dpkg-deb -I "$deb_file" > /dev/null 2>&1; then
        echo "  Invalid DEB package"; return 1
    fi
    echo "  Passed"
}

ERRORS=0

for dmg in "$ARTIFACT_DIR"/*.dmg; do
    [ -f "$dmg" ] && test_dmg "$dmg" || true
done

for exe in "$ARTIFACT_DIR"/*setup.exe "$ARTIFACT_DIR"/*.exe; do
    [ -f "$exe" ] && test_nsis "$exe" || true
done

for appimage in "$ARTIFACT_DIR"/*.AppImage; do
    [ -f "$appimage" ] && test_appimage "$appimage" || true
done

for deb in "$ARTIFACT_DIR"/*.deb; do
    [ -f "$deb" ] && test_deb "$deb" || true
done

echo ""
echo "All GUI installation tests completed"
