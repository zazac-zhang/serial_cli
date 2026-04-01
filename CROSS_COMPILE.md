# Serial CLI - Cross-Compilation Guide

Complete guide for cross-compiling Serial CLI across different platforms.

## Table of Contents

- [Overview](#overview)
- [Supported Platforms](#supported-platforms)
- [Method 1: Using Cross](#method-1-using-cross)
- [Method 2: Native Compilation](#method-2-native-compilation)
- [Method 3: Using Just](#method-3-using-just)
- [Docker Requirements](#docker-requirements)
- [Platform-Specific Notes](#platform-specific-notes)
- [Release Workflow](#release-workflow)
- [Troubleshooting](#troubleshooting)

## Overview

Serial CLI can be cross-compiled for multiple target platforms:

- **Linux**: x86_64, aarch64 (ARM64)
- **macOS**: x86_64 (Intel), arm64 (Apple Silicon)
- **Windows**: x86_64 (MSVC)

## Supported Platforms

| Platform | Target Triple | Status |
|----------|--------------|--------|
| Linux x86_64 | `x86_64-unknown-linux-gnu` | ✅ Full Support |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | ✅ Full Support |
| macOS x86_64 | `x86_64-apple-darwin` | ✅ Full Support (macOS only) |
| macOS ARM64 | `aarch64-apple-darwin` | ✅ Full Support (macOS only) |
| Windows x86_64 | `x86_64-pc-windows-msvc` | ✅ Full Support |

## Method 1: Using Cross

[cross](https://github.com/cross-rs/cross) is the recommended tool for cross-compilation. It uses Docker containers to provide consistent build environments.

### Installation

```bash
cargo install cross
```

### Docker Installation

**Linux:**
```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install docker.io

# Start Docker
sudo systemctl start docker
sudo systemctl enable docker

# Add user to docker group (optional, avoids sudo)
sudo usermod -aG docker $USER
newgrp docker
```

**macOS:**
```bash
# Install Docker Desktop
# https://www.docker.com/products/docker-desktop

# Or use Homebrew
brew install --cask docker
```

**Windows:**
- Install [Docker Desktop](https://www.docker.com/products/docker-desktop)

### Building with Cross

#### Linux x86_64

```bash
cross build --release --target x86_64-unknown-linux-gnu
```

Output: `target/x86_64-unknown-linux-gnu/release/serial-cli`

#### Linux ARM64

```bash
cross build --release --target aarch64-unknown-linux-gnu
```

Output: `target/aarch64-unknown-linux-gnu/release/serial-cli`

#### Windows x86_64

```bash
cross build --release --target x86_64-pc-windows-msvc
```

Output: `target/x86_64-pc-windows-msvc/release/serial-cli.exe`

### Cross Configuration

Create `.cross.toml` in project root:

```toml
[build]
# Default target
x86_64-unknown-linux-gnu = true

[build.env]
passthrough = [
  "RUST_LOG",
  "RUSTFLAGS"
]
```

## Method 2: Native Compilation

### Compiling on Target Platform

The simplest method is to compile directly on the target platform.

#### Linux

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release
```

#### macOS

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build
cargo build --release

# Binary supports both x86_64 and arm64 (if built on respective architecture)
```

#### Windows

```bash
# Install Rust
# https://rustup.rs/

# Build in PowerShell or Command Prompt
cargo build --release
```

### Cross-Compiling Without Docker

You can cross-compile without Docker by installing the appropriate toolchains.

#### Linux to Windows

```bash
# Add Windows target
rustup target add x86_64-pc-windows-msvc

# Install MinGW toolchain
sudo apt-get install gcc-mingw-w64-x86-64

# Build
cargo build --release --target x86_64-pc-windows-msvc
```

#### Linux to ARM64

```bash
# Add ARM64 target
rustup target add aarch64-unknown-linux-gnu

# Install ARM64 cross-compiler
sudo apt-get install gcc-aarch64-linux-gnu

# Configure cargo
mkdir -p ~/.cargo
cat >> ~/.cargo/config.toml << EOF
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
EOF

# Build
cargo build --release --target aarch64-unknown-linux-gnu
```

## Method 3: Using Just

The project includes a `justfile` with convenient cross-compilation commands.

### Install Just

```bash
cargo install just
```

### Available Commands

```bash
# Show all commands
just --list

# Build all platforms
just build-all

# Build specific platform
just build-linux
just build-macos
just build-windows

# Full release build
just release
```

### Build All Platforms

```bash
just build-all
```

This will:
1. Build Linux x86_64
2. Build Linux ARM64 (if `cross` is installed)
3. Build macOS x86_64 and ARM64 (if on macOS)
4. Build Windows x86_64 (if `cross` is installed)

## Docker Requirements

### Docker Version

- Docker 20.10 or later recommended
- Docker Compose not required

### Permissions

**Linux:**
```bash
# Check if user is in docker group
groups

# If not, add user
sudo usermod -aG docker $USER

# Log out and back in, or use:
newgrp docker
```

**macOS/Windows:**
- Docker Desktop manages permissions automatically

### Docker Resources

**Default limits:**
- Memory: 2GB (increase to 4GB+ for large projects)
- CPUs: 2 cores

**Recommended settings:**
- Memory: 4-8GB
- CPUs: 4 cores

Configure in Docker Desktop settings.

## Platform-Specific Notes

### Linux

#### Dependencies

Serial CLI requires these system libraries:
- `libudev` (for serial port enumeration)

**Ubuntu/Debian:**
```bash
sudo apt-get install build-essential libudev-dev
```

**Fedora/RHEL:**
```bash
sudo dnf install gcc make libudev-devel
```

**Arch Linux:**
```bash
sudo pacman -S base-devel systemd-libs
```

#### Serial Port Permissions

Users need permission to access serial ports:

```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER

# Log out and back in
```

### macOS

#### Xcode Command Line Tools

Required for building:

```bash
xcode-select --install
```

#### Universal Binaries

To create universal binaries (x86_64 + ARM64):

```bash
# Build both architectures
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
  target/x86_64-apple-darwin/release/serial-cli \
  target/aarch64-apple-darwin/release/serial-cli \
  -output target/release/serial-cli-universal
```

### Windows

#### Visual Studio Build Tools

Required for MSVC builds:

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. Select "C++ build tools"
3. Include Windows SDK

#### MSVC vs MinGW

- **MSVC** (`x86_64-pc-windows-msvc`): Recommended, official
- **MinGW** (`x86_64-pc-windows-gnu`): Alternative, smaller

## Release Workflow

### Automated Release

The project uses GitHub Actions for automated builds. See `.github/workflows/`.

### Manual Release

#### 1. Clean Build Directory

```bash
cargo clean
```

#### 2. Build All Platforms

```bash
just release
```

Or manually:

```bash
# Linux
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu

# Windows
cross build --release --target x86_64-pc-windows-msvc

# macOS (only on macOS)
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

#### 3. Rename Binaries

```bash
# Linux
cp target/x86_64-unknown-linux-gnu/release/serial-cli \
   serial-cli-linux-x86_64

cp target/aarch64-unknown-linux-gnu/release/serial-cli \
   serial-cli-linux-aarch64

# macOS
cp target/x86_64-apple-darwin/release/serial-cli \
   serial-cli-macos-x86_64

cp target/aarch64-apple-darwin/release/serial-cli \
   serial-cli-macos-arm64

# Windows
cp target/x86_64-pc-windows-msvc/release/serial-cli.exe \
   serial-cli-windows-x86_64.exe
```

#### 4. Create Checksums

```bash
# Generate SHA256 checksums
shasum -a 256 serial-cli-* > SHA256SUMS
```

#### 5. Test Binaries

Test each binary on the target platform or in a VM/container.

#### 6. Create GitHub Release

```bash
# Create tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

Then on GitHub:
1. Go to Releases
2. "Draft a new release"
3. Attach binaries and SHA256SUMS

## Troubleshooting

### Docker Issues

#### Permission Denied

```bash
# Error: permission denied while trying to connect to the Docker daemon
# Solution: Add user to docker group (see Docker Requirements)
```

#### Out of Memory

```bash
# Error: failed to allocate memory
# Solution: Increase Docker memory limit in Docker Desktop settings
```

#### Container Build Failures

```bash
# Error: failed to run custom build command
# Solution: Clear Docker cache
docker system prune -a
```

### Cross Compilation Issues

#### Target Not Found

```bash
# Error: error: linker `x86_64-linux-gnu-gcc` not found
# Solution: Install cross-compiler or use cross tool
cargo install cross
```

#### Library Not Found

```bash
# Error: cannot find -ludev
# Solution: Install libudev-dev in Docker container or system
```

#### Symbol Stripping Issues

```bash
# Error: failed to strip
# Solution: Strip manually
strip target/x86_64-unknown-linux-gnu/release/serial-cli
```

### Platform-Specific Issues

#### macOS: Code Signing

```bash
# Warning: binary not signed
# Solution: Sign binary (optional, for distribution)
codesign --force --deep --sign - serial-cli
```

#### Windows: Missing DLL

```bash
# Error: VCRUNTIME140.dll not found
# Solution: Install Visual C++ Redistributable
# https://aka.ms/vs/17/release/vc_redist.x64.exe
```

#### Linux: glibc Version

```bash
# Error: version 'GLIBC_2.29' not found
# Solution: Build on older system or use static linking
```

### Build Verification

#### Check Binary Type

```bash
# Linux/macOS
file serial-cli

# Output examples:
# serial-cli: ELF 64-bit LSB executable, x86-64
# serial-cli: Mach-O 64-bit executable x86_64
# serial-cli: Mach-O 64-bit executable arm64
# serial-cli.exe: PE32+ executable (console) x86-64
```

#### Check Architecture

```bash
# Linux
readelf -h serial-cli | grep Machine

# macOS
lipo -info serial-cli

# Windows
# Check file properties or use:
dumpbin /HEADERS serial-cli.exe
```

#### Test Binary

```bash
# Test help command
./serial-cli --help

# Test list ports
./serial-cli list-ports
```

## Additional Resources

- [cross Documentation](https://github.com/cross-rs/cross)
- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Cargo Targets](https://doc.rust-lang.org/cargo/appendix/glossary.html#target)
- [DEVELOPMENT.md](DEVELOPMENT.md) - Development guide
- [USAGE.md](USAGE.md) - Usage documentation

## Getting Help

If you encounter issues:

1. Check this guide's troubleshooting section
2. Search [GitHub Issues](https://github.com/yourusername/serial-cli/issues)
3. Create a new issue with:
   - Platform and target triple
   - Error message
   - Build command used
   - Docker version (if applicable)

---

**Happy Cross-Compiling!** 🌍
