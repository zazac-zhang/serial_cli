#!/usr/bin/env bash
set -euo pipefail

# Serial CLI Release Script
# Usage: ./scripts/release.sh <version> [--check-only] [--no-checks]
# Examples:
#   ./scripts/release.sh v1.2.3          # Run checks, then prepare release
#   ./scripts/release.sh v1.2.3 --check-only  # Only run checks, no version bump
#   ./scripts/release.sh v1.2.3 --no-checks   # Only bump version, skip checks

VERSION="${1:-}"
CHECK_ONLY=false
NO_CHECKS=false

for arg in "${@:2}"; do
    case "$arg" in
        --check-only) CHECK_ONLY=true ;;
        --no-checks) NO_CHECKS=true ;;
        *) echo "Unknown option: $arg"; exit 1 ;;
    esac
done

if [ -z "$VERSION" ]; then
    echo "Usage: release.sh <version> [--check-only] [--no-checks]"
    echo "Example: release.sh v1.2.3"
    exit 1
fi

# Remove 'v' prefix if present
VERSION_NUM="${VERSION#v}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

check() {
    local name="$1"
    local command="$2"
    local critical="${3:-true}"
    echo -n "  $name... "
    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}PASS${NC}"
    else
        if [ "$critical" = "true" ]; then
            echo -e "${RED}FAIL${NC}"
            return 1
        else
            echo -e "${YELLOW}WARN${NC}"
        fi
    fi
}

run_checks() {
    echo "Running pre-release checks..."
    echo ""
    local failed=0

    echo "Code Quality:"
    check "Clean git status" "git diff --quiet" || ((failed++))
    check "All tests pass" "cargo test --workspace" || ((failed++))
    check "Code formatting" "cargo fmt --check" || ((failed++))
    check "Clippy lints" "cargo clippy -- -D warnings" || ((failed++))
    echo ""

    echo "Build:"
    check "Release build" "cargo build --release" || ((failed++))
    check "Binary execution" "cargo run --release -- --help" || ((failed++))
    echo ""

    echo "Documentation:"
    check "README.md exists" "test -f README.md" false
    check "CHANGELOG.md exists" "test -f CHANGELOG.md" false
    echo ""

    echo "Security:"
    check "License exists" "test -f LICENSE" false
    check "No sensitive data" "! git grep -iE 'password|secret|api_key' -- '*.rs' '*.toml' '*.json'" || ((failed++))
    echo ""

    if [ "$failed" -gt 0 ]; then
        echo -e "${RED}❌ $failed check(s) failed. Aborting.${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ All checks passed.${NC}"
}

bump_version() {
    echo "Preparing release $VERSION..."
    echo ""

    # Update Cargo.toml
    local current_version
    current_version=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    sed -i '' "s/^version = \"$current_version\"/version = \"$VERSION_NUM\"/" Cargo.toml
    echo "  Cargo.toml: $current_version -> $VERSION_NUM"

    # Update GUI version if src-tauri exists
    if [ -f src-tauri/Cargo.toml ]; then
        local gui_version="${VERSION_NUM}-gui.0"
        sed -i '' "s/^version = \".*\"/version = \"$gui_version\"/" src-tauri/Cargo.toml
        echo "  src-tauri/Cargo.toml: -> $gui_version"
    fi

    # Update frontend package.json if exists
    if [ -f frontend/package.json ]; then
        local gui_version="${VERSION_NUM}-gui.0"
        sed -i '' "s/\"version\": \".*\"/\"version\": \"$gui_version\"/" frontend/package.json
        echo "  frontend/package.json: -> $gui_version"
    fi

    # Generate changelog
    if command -v git-cliff &> /dev/null; then
        git-cliff --tag "$VERSION" --output CHANGELOG.md
        echo "  Generated CHANGELOG.md"
    fi

    echo ""
    echo "Release prepared. Review and commit:"
    echo "  git commit -am \"chore: prepare release $VERSION\""
}

if [ "$NO_CHECKS" = false ]; then
    run_checks
fi

if [ "$CHECK_ONLY" = false ]; then
    bump_version
fi
