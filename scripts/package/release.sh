#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"

if [ -z "$VERSION" ]; then
    echo "Usage: release.sh <version>"
    echo "Example: release.sh v1.2.3"
    exit 1
fi

echo "Creating release $VERSION..."

# Check if on master/main branch
CURRENT_BRANCH=$(git branch --show-current)
if [[ "$CURRENT_BRANCH" != "master" && "$CURRENT_BRANCH" != "main" ]]; then
    echo "⚠ Warning: Not on master/main branch"
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠ Warning: You have uncommitted changes"
    git status --short
    read -p "Continue anyway? (y/n) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Create tag
git tag -a "$VERSION" -m "Release $VERSION"
echo "Created tag $VERSION"

# Push tag
read -p "Push tag $VERSION to remote? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    git push origin "$VERSION"
    echo "✓ Tag pushed. GitHub Actions will handle the release."
else
    echo "Tag created locally. Push manually with: git push origin $VERSION"
fi
