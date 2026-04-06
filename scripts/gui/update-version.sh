#!/usr/bin/env bash
set -euo pipefail

CLI_VERSION="${1:-}"
GUI_INCREMENT="${2:-patch}"

if [ -z "$CLI_VERSION" ]; then
    echo "Usage: update-version.sh <cli_version> [gui_increment]"
    echo "Example: update-version.sh v1.2.3 patch"
    exit 1
fi

# Parse CLI version (remove 'v' prefix)
CLI_VERSION_NUM="${CLI_VERSION#v}"
MAJOR=$(echo $CLI_VERSION_NUM | cut -d. -f1)
MINOR=$(echo $CLI_VERSION_NUM | cut -d. -f2)
PATCH=$(echo $CLI_VERSION_NUM | cut -d. -f3)

# Get current GUI patch number
GUI_PATCH=0
LAST_TAG=$(git tag --list "v${MAJOR}.${MINOR}.*-gui.*" | sort -V | tail -1 || echo "")
if [ -n "$LAST_TAG" ]; then
    CURRENT_GUI_PATCH=$(echo "$LAST_TAG" | sed 's/.*-gui\.//' | sed 's/^0*//')
    if [ "$GUI_INCREMENT" = "patch" ]; then
        GUI_PATCH=$((CURRENT_GUI_PATCH + 1))
    elif [ "$GUI_INCREMENT" = "reset" ]; then
        GUI_PATCH=0
    else
        GUI_PATCH=$CURRENT_GUI_PATCH
    fi
fi

GUI_VERSION="v${MAJOR}.${MINOR}.${PATCH}-gui.${GUI_PATCH}"

echo "Updating GUI version to $GUI_VERSION"
echo "  Based on CLI version: v${CLI_VERSION_NUM}"
echo "  GUI patch level: $GUI_PATCH"

# Update src-tauri/Cargo.toml
if [ -f src-tauri/Cargo.toml ]; then
    sed -i.bak "s/^version = \".*\"/version = \"${GUI_VERSION#v}\"/" src-tauri/Cargo.toml
    rm -f src-tauri/Cargo.toml.bak
    echo "✓ Updated src-tauri/Cargo.toml"
else
    echo "⚠ src-tauri/Cargo.toml not found"
fi

# Update src-ui/package.json
if [ -f src-ui/package.json ]; then
    cd src-ui
    npm version --no-git-tag-version "${GUI_VERSION#v}"
    cd ..
    echo "✓ Updated src-ui/package.json"
else
    echo "⚠ src-ui/package.json not found"
fi

echo ""
echo "Version updated to $GUI_VERSION"
echo "Commit with: git commit -am \"chore: bump GUI version to $GUI_VERSION\""
