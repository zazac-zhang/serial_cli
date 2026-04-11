#!/usr/bin/env bash
set -euo pipefail

# Serial CLI v1.0.0 Release Checklist
# This script performs all pre-release checks

VERSION="v1.0.0"
echo "🚀 Serial CLI $VERSION Release Checklist"
echo "========================================"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Counter
PASS=0
FAIL=0
WARN=0

# Function to check and report
check() {
    local name="$1"
    local command="$2"
    local critical="${3:-true}"

    echo -n "Checking $name... "
    if eval "$command" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASS${NC}"
        ((PASS++))
        return 0
    else
        if [ "$critical" = "true" ]; then
            echo -e "${RED}✗ FAIL${NC}"
            ((FAIL++))
            return 1
        else
            echo -e "${YELLOW}⚠ WARN${NC}"
            ((WARN++))
            return 0
        fi
    fi
}

echo "📋 Phase 1: Code Quality Checks"
echo "--------------------------------"

check "Clean git status" "git diff --quiet"
check "On master/main branch" "[ $(git branch --show-current) = 'master' ] || [ $(git branch --show-current) = 'main' ]"
check "No uncommitted changes" "git diff --quiet"
check "All tests pass" "cargo test --workspace"
check "Zero compilation warnings" "! cargo build 2>&1 | grep -q 'warning:'"
check "Code formatting" "cargo fmt --check"
check "Clippy lints" "cargo clippy -- -D warnings"

echo ""
echo "🏗️  Phase 2: Build Verification"
echo "--------------------------------"

check "Release build" "cargo build --release"
check "Binary execution" "cargo run --release -- --help"

echo ""
echo "📚 Phase 3: Documentation Checks"
echo "--------------------------------"

check "README.md exists" "test -f README.md"
check "CHANGELOG.md updated" "grep -q '$VERSION' CHANGELOG.md"
check "All docs referenced exist" "bash -c '! grep -r \"TODO_CLI\|TODO_UI\|CODE_REVIEW_DETAILED\" *.md docs/*.md 2>/dev/null'"

echo ""
echo "🧪 Phase 4: Testing Verification"
echo "--------------------------------"

check "Unit tests" "cargo test --lib"
check "Integration tests" "cargo test --test integration_tests"
check "Lua integration tests" "cargo test --test lua_integration_tests"
check "Concurrency stress tests" "cargo test --test concurrency_stress_tests"

echo ""
echo "🎯 Phase 5: Performance Benchmarks"
echo "----------------------------------"

check "Performance benchmarks run" "cargo bench --bench performance"

echo ""
echo "📦 Phase 6: Release Preparation"
echo "--------------------------------"

check "Version number consistency" "grep -q 'version = \"1.0.0\"' Cargo.toml"
check "Release script exists" "test -f scripts/package/release.sh"
check "GitHub Actions workflow" "test -f .github/workflows/release.yml"

echo ""
echo "🌐 Phase 7: Cross-Platform Readiness"
echo "-------------------------------------"

check "Linux build support" "grep -q 'x86_64-unknown-linux-gnu' .github/workflows/release.yml"
check "macOS build support" "grep -q 'x86_64-apple-darwin' .github/workflows/release.yml"
check "Windows build support" "grep -q 'x86_64-pc-windows-msvc' .github/workflows/release.yml"
check "ARM64 build support" "grep -q 'aarch64' .github/workflows/release.yml"

echo ""
echo "🔐 Phase 8: Security and Legal"
echo "-------------------------------"

check "License file exists" "test -f LICENSE"
check "No sensitive data" "! git grep -i 'password\|secret\|api_key' | grep -v node_modules | grep -v '.git'"

echo ""
echo "========================================"
echo "📊 Release Readiness Summary"
echo "========================================"
echo -e "${GREEN}PASSED: $PASS${NC}"
echo -e "${YELLOW}WARNINGS: $WARN${NC}"
echo -e "${RED}FAILED: $FAIL${NC}"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}✅ Ready for $VERSION release!${NC}"
    echo ""
    echo "Next steps:"
    echo "1. Review warnings if any"
    echo "2. Run: ./scripts/package/prepare-release.sh $VERSION"
    echo "3. Review and commit changes"
    echo "4. Run: ./scripts/package/release.sh $VERSION"
    echo "5. Push tag to trigger GitHub Actions"
    exit 0
else
    echo -e "${RED}❌ Not ready for release. Please fix the $FAIL failed check(s) above.${NC}"
    exit 1
fi