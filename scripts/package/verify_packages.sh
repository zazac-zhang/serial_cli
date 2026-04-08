#!/usr/bin/env bash
set -euo pipefail

echo "Verifying package manager configurations..."

ERRORS=0

# Check Homebrew script
if [ -f scripts/package/update_homebrew.sh ]; then
    if bash -n scripts/package/update_homebrew.sh; then
        echo "✓ Homebrew script syntax OK"
    else
        echo "✗ Homebrew script syntax error"
        ERRORS=$((ERRORS + 1))
    fi
else
    echo "✗ Homebrew script missing"
    ERRORS=$((ERRORS + 1))
fi

# Check Scoop script
if [ -f scripts/package/update_scoop.sh ]; then
    if bash -n scripts/package/update_scoop.sh; then
        echo "✓ Scoop script syntax OK"
    else
        echo "✗ Scoop script syntax error"
        ERRORS=$((ERRORS + 1))
    fi
else
    echo "✗ Scoop script missing"
    ERRORS=$((ERRORS + 1))
fi

# Check AUR script
if [ -f scripts/package/generate_aur.sh ]; then
    if bash -n scripts/package/generate_aur.sh; then
        echo "✓ AUR script syntax OK"
    else
        echo "✗ AUR script syntax error"
        ERRORS=$((ERRORS + 1))
    fi
else
    echo "✗ AUR script missing"
    ERRORS=$((ERRORS + 1))
fi

# Check version in Cargo.toml
if grep -q '^version = "[0-9]' Cargo.toml; then
    echo "✓ Cargo.toml version format OK"
else
    echo "✗ Cargo.toml version format invalid"
    ERRORS=$((ERRORS + 1))
fi

# Check license in Cargo.toml
if grep -q 'license' Cargo.toml; then
    echo "✓ Cargo.toml license defined"
else
    echo "⚠ Cargo.toml license not defined"
fi

if [ $ERRORS -gt 0 ]; then
    echo ""
    echo "✗ Verification failed with $ERRORS error(s)"
    exit 1
else
    echo ""
    echo "✓ All package manager checks passed"
fi
