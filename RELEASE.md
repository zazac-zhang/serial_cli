# Release Guide

## Prerequisites

- Rust toolchain installed
- git-cliff: `cargo install git-cliff`
- Write access to GitHub Repository

## Release Process

### 1. Prepare Release

```bash
# Check readiness
./scripts/release.sh v1.2.3 --check-only

# Prepare new version (runs checks then updates versions)
./scripts/release.sh v1.2.3

# Or skip checks if you already verified
./scripts/release.sh v1.2.3 --no-checks

# Review changes
git diff
git status

# Commit version changes
git commit -am "chore: prepare release v1.2.3"
```

### 2. Create Release

```bash
# Create and push tag
git tag -a v1.2.3 -m "Release v1.2.3"
git push origin v1.2.3
```

After pushing the tag, GitHub Actions will:
1. Build binaries for all platforms
2. Create GitHub Release
3. Update Homebrew, Scoop, and AUR
4. Publish to crates.io

### 3. Verify Release

- [ ] GitHub Release created
- [ ] All platform builds successful
- [ ] crates.io publish successful
- [ ] CHANGELOG.md updated

## Rollback

If release fails or issues are found:

```bash
# Delete GitHub Release and tag
gh release delete v1.2.3 --cleanup-tag

# Delete local tag
git tag -d v1.2.3

# Fix issues and re-release
```

## Conventional Commits

Commit message format:

```
<type>(<scope>): <subject>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Code style
- `refactor`: Refactoring
- `perf`: Performance
- `test`: Tests
- `chore`: Build/tools
- `ci`: CI/CD

**Examples:**
```bash
git commit -m "feat(cli): add protocol list command"
git commit -m "fix(protocol): handle empty response correctly"
git commit -m "docs(readme): update installation instructions"
```
