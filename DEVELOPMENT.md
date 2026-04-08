# Serial CLI - Development Guide

Developer documentation for contributing to Serial CLI.

## Table of Contents

- [Development Setup](#development-setup)
- [Build Commands](#build-commands)
- [Testing](#testing)
- [Code Quality](#code-quality)
- [Cross-Compilation](#cross-compilation)
- [Contributing](#contributing)
- [Project Structure](#project-structure)

---

## Development Setup

### Prerequisites

```bash
# Rust 1.75+
rustup update stable
rustup component add rustfmt clippy

# Just task runner (recommended)
cargo install just
```

### Platform Dependencies

**Linux:**
```bash
sudo apt-get install build-essential libudev-dev
sudo usermod -a -G dialout $USER  # Serial port access
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
- Install Visual Studio Build Tools with C++ tools
- Install USB-to-serial drivers (FTDI, CP210x, CH340)

### IDE Setup

**VS Code Recommended Extensions:**
- rust-analyzer
- CodeLLDB
- Even Better TOML
- Error Lens

**.vscode/settings.json:**
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.cargo.loadOutDirsFromCheck": true
}
```

---

## Build Commands

```bash
# Development build
just dev        # cargo build

# Release build
just build      # cargo build --release

# Run application
just run <args> # cargo run -- <args>

# Clean build artifacts
just clean
```

---

## Testing

```bash
# Run all tests
just test

# Run tests with output
just test-verbose

# Run specific test
just test <test_name>

# Watch mode (run tests on file changes)
just test-watch
```

**Test Status:** 58/58 passing

---

## Code Quality

```bash
# Format code
just fmt         # cargo fmt

# Check formatting
just fmt-check   # cargo fmt -- --check

# Run linter
just lint        # cargo clippy -- -D warnings

# Run all checks (format + lint + test)
just check
```

**All checks must pass before committing.**

---

## Cross-Compilation

### Prerequisites

```bash
# Install cross for cross-compilation
cargo install cross

# Docker required (cross uses containers)
```

### Build Commands

```bash
# All platforms
just build-all

# Specific platforms
just build-linux    # x86_64 + aarch64
just build-macos    # x86_64 + arm64 (macOS only)
just build-windows  # x86_64 (requires cross)

# Full release build (clean + all platforms)
just release
```

---

## GUI Development

### Prerequisites

```bash
# Node.js 20+
# Rust + Tauri CLI
cargo install tauri-cli
```

### Commands

```bash
# Install frontend dependencies
just gui-deps

# Start development server
just gui-dev

# Build GUI application
just gui-build

# Type check frontend
just gui-type-check

# Check Rust code
just gui-check

# Format all code
just gui-fmt

# Clean build artifacts
just gui-clean
```

---

## Contributing

### Contribution Types

- 🐛 Bug fixes
- ✨ New features
- 🧪 Tests
- 📚 Documentation
- 🔧 Performance improvements

### Pull Request Process

```bash
# 1. Create a branch
git checkout -b feature/your-feature-name

# 2. Run all checks
just check

# 3. Commit your changes
git commit -m "Add: Your feature description"

# 4. Create PR
```

### Commit Message Format

```
<type>: <short description>
```

**Types:** `Add:`, `Fix:`, `Update:`, `Refactor:`, `Docs:`, `Test:`, `Chore:`, `Perf:`

**Examples:**
```bash
git commit -m "Add: Modbus RTU protocol support"
git commit -m "Fix: Handle empty response in AT protocol"
git commit -m "Docs: Update installation instructions"
```

### Code Style

- Use `cargo fmt` for formatting
- Fix all `clippy` warnings
- Write unit tests for new features
- Add comments for complex logic

---

## Project Structure

```
serial_cli/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root (re-exports Result, SerialError)
│   ├── error.rs             # Error types (SerialError, Result<T>)
│   ├── config.rs            # Configuration management
│   ├── serial_core/         # Serial port I/O
│   │   ├── port.rs          # PortManager, SerialConfig, PortHandle
│   │   ├── io_loop.rs       # Async I/O event loop
│   │   └── sniffer.rs       # Packet capture/monitoring
│   ├── protocol/            # Protocol engine
│   │   ├── registry.rs      # ProtocolRegistry, ProtocolFactory
│   │   ├── built_in/        # Modbus, AT Command, Line protocols
│   │   ├── lua_ext.rs       # Custom Lua protocol support
│   │   ├── loader.rs        # ProtocolLoader for .lua protocols
│   │   └── validator.rs     # ProtocolValidator
│   ├── lua/                 # LuaJIT integration
│   │   ├── bindings.rs      # LuaBindings - Rust→Lua API
│   │   ├── engine.rs        # LuaEngine runtime
│   │   ├── executor.rs      # ScriptEngine execution
│   │   └── stdlib.rs        # Standard library functions
│   ├── task/                # Task scheduling
│   │   ├── queue.rs         # TaskQueue
│   │   ├── executor.rs      # TaskExecutor
│   │   └── monitor.rs       # TaskMonitor
│   └── cli/                 # CLI interface
│       ├── interactive.rs   # REPL shell
│       ├── commands.rs      # Single commands (list-ports, send)
│       ├── batch.rs         # Batch script execution
│       └── json.rs          # JSON output formatting
├── src-tauri/               # Tauri application (GUI backend)
│   ├── src/                 # Tauri-specific code
│   ├── Cargo.toml
│   ├── tauri.conf.json      # Tauri configuration
│   └── build.rs
├── frontend/                # React frontend (GUI)
│   ├── src/                 # React source
│   ├── components/          # UI components
│   ├── contexts/            # React contexts
│   ├── hooks/               # Custom hooks
│   ├── index.html
│   └── package.json
├── examples/                # Lua script examples
├── tests/                   # Integration tests
├── docs/                    # Documentation
│   └── TROUBLESHOOTING.md   # Detailed troubleshooting
├── justfile                 # Just commands
├── Cargo.toml               # Package configuration
└── README.md                # Quick start guide
```

### Core Modules

| Module | Description |
|--------|-------------|
| `serial_core` | Serial port I/O, port management |
| `protocol` | Modbus, AT Commands, custom protocols |
| `lua` | LuaJIT integration, script execution |
| `cli` | CLI interface, interactive mode |
| `task` | Task scheduling and execution |

---

## Debugging

```bash
# Debug logging
RUST_LOG=debug cargo run -- list-ports
RUST_LOG=trace cargo run -- list-ports

# Profiling
cargo install flamegraph
cargo flamegraph --bin serial-cli -- list-ports

# Benchmark
cargo bench
```

---

## Architecture Overview

### Module Dependencies

```
main.rs → cli/* → serial_core → protocol/*
                    ↓
                 lua/* → protocol/* (for custom protocols)
```

### Key Design Patterns

1. **Protocol Trait** (`src/protocol/mod.rs:24`): All protocols implement `parse()`, `encode()`, `reset()`
2. **PortManager** (`src/serial_core/port.rs`): Centralized port lifecycle management with UUID-based handles
3. **LuaBindings** (`src/lua/bindings.rs`): Registers Rust APIs to Lua (`serial.open`, `protocol_encode`, etc.)
4. **Error Handling**: Centralized in `error.rs` using `thiserror`; all functions return `Result<T>`

### Key Conventions

- **Error handling**: Use `Result<T>` from `error.rs`; `SerialError::Io` wraps `std::io::Error`
- **Async**: All I/O uses `tokio`; main entry uses `#[tokio::main]`
- **Lua integration**: Scripts executed via `LuaBindings::register_all_apis()` + `execute_script()`
- **Protocol loading**: Custom protocols loaded via `protocol_load(path)` in Lua scripts
- **Configuration**: TOML-based config with fallback defaults (`config.rs`)

---

## Resources

- [Rust Guidelines](https://rust-lang.github.io/api-guidelines/)
- [API Documentation](https://docs.rs/serial-cli/)
- [README.md](README.md) - Quick start
- [GitHub Issues](https://github.com/zazac-zhang/serial_cli/issues)
