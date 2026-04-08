#!/usr/bin/env bash
set -euo pipefail

echo "Running release integration tests..."

# Test 1: Verify build script works
echo "Testing build script..."
if scripts/package/build.sh; then
    echo "✓ Build script works"
else
    echo "✗ Build script failed"
    exit 1
fi

# Test 2: Verify binary is created
echo "Testing binary creation..."
if [ -f target/release/serial-cli ]; then
    echo "✓ Binary created"
else
    echo "✗ Binary not created"
    exit 1
fi

# Test 3: Verify binary is executable
echo "Testing binary execution..."
if target/release/serial-cli --version; then
    echo "✓ Binary is executable"
else
    echo "✗ Binary execution failed"
    exit 1
fi

# Test 4: Verify workflow files are valid YAML
echo "Testing workflow YAML syntax..."
for workflow in .github/workflows/*.yml; do
    if command -v yq &> /dev/null; then
        if yq eval '.' "$workflow" > /dev/null; then
            echo "✓ $workflow is valid YAML"
        else
            echo "✗ $workflow has invalid YAML"
            exit 1
        fi
    else
        echo "⚠ yq not installed, skipping YAML validation"
        break
    fi
done

echo ""
echo "✓ All release integration tests passed"
