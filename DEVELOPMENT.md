# Serial CLI - Development Guide

Development documentation for contributors and maintainers.

## Table of Contents

- [Development Environment](#development-environment)
- [Building](#building)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Cross-Compilation](#cross-compilation)
- [Contributing](#contributing)
- [Release Process](#release-process)
- [Project Structure](#project-structure)

## Development Environment

### Prerequisites

- **Rust**: 1.70 or later
  ```bash
  rustup update stable
  rustup component add rustfmt clippy
  ```

- **Git**: For version control
  ```bash
  git clone https://github.com/yourusername/serial-cli.git
  cd serial-cli
  ```

- **Just**: Command runner (optional but recommended)
  ```bash
  cargo install just
  ```

- **Make**: Alternative to Just (if you don't want to use Just)

### Recommended Tools

- **Editor**: VS Code with rust-analyzer extension
- **Linter**: `cargo clippy`
- **Formatter**: `rustfmt`
- **Documentation**: `cargo doc`

### IDE Setup

#### VS Code

Install extensions:
- `rust-analyzer`
- `CodeLLDB` (for debugging)
- `Even Better TOML` (for Cargo.toml)
- `Error Lens` (for inline errors)

#### Workspace Configuration

Create `.vscode/settings.json`:
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.loadOutDirsFromCheck": true,
  "files.watcherExclude": {
    "**/target/**": true
  }
}
```

## Building

### Development Build

```bash
# Using Just
just dev

# Using Cargo
cargo build
```

### Release Build

```bash
# Using Just
just build

# Using Cargo
cargo build --release
```

The release binary will be at `target/release/serial-cli`.

### Build Features

- **Optimization Level**: 3 (maximum)
- **LTO**: Enabled (Link-Time Optimization)
- **Strip Symbols**: Enabled (smaller binary)
- **Result**: ~1.6MB binary

## Testing

### Run All Tests

```bash
# Using Just
just test

# Using Cargo
cargo test
```

### Run Tests with Output

```bash
# Using Just
just test-verbose

# Using Cargo
cargo test -- --nocapture
```

### Run Specific Test

```bash
# Using Just
just test-test test_name

# Using Cargo
cargo test test_name
```

### Test Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

Current test status: **58/58 passing** ✅

## Code Quality

### Format Code

```bash
# Using Just
just fmt

# Using Cargo
cargo fmt
```

### Check Formatting

```bash
# Using Just
just fmt-check

# Using Cargo
cargo fmt -- --check
```

### Lint with Clippy

```bash
# Using Just
just lint

# Using Cargo
cargo clippy -- -D warnings
```

### Run All Checks

```bash
# Using Just
just check

# Manually
cargo fmt -- --check
cargo clippy -- -D warnings
cargo test
```

All checks must pass before committing.

## Cross-Compilation

### Using Cross Tool

```bash
# Install cross
cargo install cross

# Build for Linux x86_64
cross build --release --target x86_64-unknown-linux-gnu

# Build for Linux ARM64
cross build --release --target aarch64-unknown-linux-gnu

# Build for Windows
cross build --release --target x86_64-pc-windows-msvc
```

### Using Just

```bash
# Build all platforms
just build-all

# Build specific platform
just build-linux
just build-macos
just build-windows
```

See [CROSS_COMPILE.md](CROSS_COMPILE.md) for detailed cross-compilation guide.

## Contributing

### Workflow

1. **Fork** the repository
2. **Create branch** from `main`
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make changes** following code style
4. **Run checks**
   ```bash
   just check
   ```
5. **Commit** with clear message
   ```bash
   git commit -m "Add: Your feature description"
   ```
6. **Push** to your fork
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Create Pull Request** on GitHub

### Commit Message Style

Use conventional commits:

- `Add:` - New features
- `Fix:` - Bug fixes
- `Update:` - Updates to existing features
- `Refactor:` - Code refactoring
- `Docs:` - Documentation changes
- `Test:` - Adding or updating tests
- `Chore:` - Maintenance tasks

Examples:
```
Add: Support for custom Lua protocols
Fix: Correct timeout handling in recv command
Refactor: Simplify error handling in serial_core
Docs: Update USAGE.md with new examples
```

### Code Style

Follow Rust best practices:

- Use `cargo fmt` for formatting
- Fix all `clippy` warnings
- Add tests for new features
- Update documentation for public APIs
- Keep functions focused and small
- Use meaningful variable names
- Add comments for complex logic

### Adding Features

1. **Design** first - discuss in issue if major feature
2. **Implement** with tests
3. **Document** - update USAGE.md if user-facing
4. **Test** manually and with unit tests
5. **Update** CHANGELOG.md (if it exists)

### Reporting Issues

Include:
- Rust version (`rustc --version`)
- Operating system
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs
- Minimal reproducible example if possible

## Release Process

### Version Bump

1. Update version in `Cargo.toml`
   ```toml
   [package]
   version = "0.2.0"
   ```

2. Update version in `README.md` and `USAGE.md`

3. Commit version bump
   ```bash
   git commit -m "Bump: Version 0.2.0"
   ```

4. Create git tag
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```

### Build Release Binaries

```bash
# Build all platforms
just release

# Binaries will be in:
# target/x86_64-unknown-linux-gnu/release/
# target/x86_64-apple-darwin/release/
# target/aarch64-apple-darwin/release/
# target/x86_64-pc-windows-msvc/release/
```

### Create GitHub Release

1. Go to GitHub Releases
2. Click "Draft a new release"
3. Tag: `v0.2.0`
4. Title: `Release v0.2.0`
5. Description: Include changelog
6. Attach binaries:
   - `serial-cli-linux-x86_64`
   - `serial-cli-linux-aarch64`
   - `serial-cli-macos-x86_64`
   - `serial-cli-macos-arm64`
   - `serial-cli-windows-x86_64.exe`

### Verify Release

1. Download and test binaries on each platform
2. Run all tests
3. Verify installation instructions
4. Check documentation links

## Project Structure

```
serial-cli/
├── src/
│   ├── main.rs                 # CLI entry point
│   ├── lib.rs                  # Library entry point
│   ├── error.rs                # Error types
│   ├── config.rs               # Configuration management
│   ├── serial_core/            # Serial port core
│   │   ├── port.rs             # Port management
│   │   ├── io_loop.rs          # Async I/O loop
│   │   └── sniffer.rs          # Packet sniffer
│   ├── protocol/               # Protocol engine
│   │   ├── registry.rs         # Protocol registry
│   │   ├── built_in/           # Built-in protocols
│   │   └── lua_ext.rs          # Lua protocol extensions
│   ├── lua/                    # Lua runtime
│   │   ├── engine.rs           # Lua engine
│   │   ├── bindings.rs         # API bindings
│   │   ├── executor.rs         # Script executor
│   │   └── stdlib.rs           # Standard library
│   ├── task/                   # Task scheduling
│   │   ├── queue.rs            # Task queue
│   │   ├── executor.rs         # Task executor
│   │   └── monitor.rs          # Task monitor
│   └── cli/                    # CLI interface
│       ├── interactive.rs      # Interactive shell
│       ├── commands.rs         # Single commands
│       ├── batch.rs            # Batch processing
│       └── json.rs             # JSON output
├── examples/                   # Lua script examples
├── tests/                      # Integration tests
├── docs/                       # Documentation
├── config/                     # Default configuration
├── justfile                    # Just commands
├── Cargo.toml                  # Package manifest
├── README.md                   # Quick start
├── USAGE.md                    # Usage guide
├── DEVELOPMENT.md              # This file
├── CROSS_COMPILE.md            # Cross-compilation guide
└── .github/workflows/          # CI/CD workflows
```

### Key Modules

#### `serial_core/`
Core serial port functionality.
- `port.rs`: Port management, configuration
- `io_loop.rs`: Async I/O event loop
- `sniffer.rs`: Packet monitoring

#### `protocol/`
Protocol encoding/decoding.
- `registry.rs`: Protocol factory and registry
- `built_in/`: Modbus, AT Command, Line protocols
- `lua_ext.rs`: Custom Lua protocols

#### `lua/`
LuaJIT integration.
- `engine.rs`: Lua runtime setup
- `bindings.rs`: Rust-to-Lua API bindings
- `stdlib.rs`: Lua standard library functions

#### `cli/`
Command-line interface.
- `interactive.rs`: REPL shell
- `commands.rs`: Single-shot commands
- `batch.rs`: Batch script execution
- `json.rs`: JSON output formatting

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run -- list-ports
```

### Enable Trace Logging

```bash
RUST_LOG=trace cargo run -- list-ports
```

### Using LLDB (VS Code)

Create `.vscode/launch.json`:
```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug serial-cli",
      "cargo": {
        "args": ["build"],
        "filter": {
          "name": "serial-cli",
          "kind": "bin"
        }
      },
      "args": ["list-ports"],
      "env": {
        "RUST_LOG": "debug"
      }
    }
  ]
}
```

## Performance Optimization

### Profiling

```bash
# Install flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bin serial-cli -- list-ports
```

### Benchmarking

```bash
# Run benchmarks
cargo bench
```

### Optimization Tips

- Use `--release` for testing performance
- Profile before optimizing
- Check for allocations in hot paths
- Consider async for I/O-bound operations
- Use LuaJIT for fast script execution

## Additional Resources

- [Rust Guidelines](https://rust-lang.github.io/api-guidelines/)
- [API Documentation](https://docs.rs/serial-cli/)
- [USAGE.md](USAGE.md) - User documentation
- [CROSS_COMPILE.md](CROSS_COMPILE.md) - Cross-compilation

## Getting Help

- GitHub Issues: https://github.com/yourusername/serial-cli/issues
- Discussions: https://github.com/yourusername/serial-cli/discussions

---

**Happy Coding!** 🦀
