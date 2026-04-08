#!/usr/bin/env bash
set -euo pipefail

echo "Verifying release configuration..."

ERRORS=0

# Check Cargo.toml version format
if grep -q '^version = "[0-9]' Cargo.toml; then
    echo "✓ Cargo.toml version format OK"
else
    echo "✗ Cargo.toml version format invalid"
    ERRORS=$((ERRORS + 1))
fi

# Check for CHANGELOG.md
if [ -f CHANGELOG.md ]; then
    echo "✓ CHANGELOG.md exists"
else
    echo "⚠ CHANGELOG.md not found (will be generated)"
fi

# Check workflow files
for workflow in commit-lint version-bump release; do
    if [ -f ".github/workflows/${workflow}.yml" ]; then
        echo "✓ .github/workflows/${workflow}.yml exists"
    else
        echo "✗ .github/workflows/${workflow}.yml missing"
        ERRORS=$((ERRORS + 1))
    fi
done

# Check config files
if [ -f .github/commitlint.config.js ]; then
    echo "✓ commitlint.config.js exists"
else
    echo "✗ commitlint.config.js missing"
    ERRORS=$((ERRORS + 1))
fi

if [ -f .github/cliff.toml ]; then
    echo "✓ git-cliff cliff.toml exists"
else
    echo "✗ git-cliff cliff.toml missing"
    ERRORS=$((ERRORS + 1))
fi

if [ $ERRORS -gt 0 ]; then
    echo ""
    echo "✗ Verification failed with $ERRORS error(s)"
    exit 1
else
    echo ""
    echo "✓ All checks passed"
fi
