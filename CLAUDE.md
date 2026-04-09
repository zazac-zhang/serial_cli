# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Serial CLI is a Rust-based serial port communication tool with embedded LuaJIT scripting, optimized for AI/automation workflows. It supports multiple protocols (Modbus RTU/ASCII, AT Commands, line-based, and custom Lua protocols) with structured JSON output.

Includes a Tauri-based GUI application (`src-tauri/` + `frontend/`).

## Build & Development Commands

```bash
# Development build
just dev          # cargo build

# Release build
just build        # cargo build --release

# Run tests
just test         # cargo test
just test-verbose # cargo test -- --nocapture

# Code quality
just check        # fmt-check + lint + test (all checks)
just fmt          # cargo fmt
just lint         # cargo clippy -- -D warnings

# Run application
just run <args>   # cargo run -- <args>

# Cross-compilation
just build-all    # Linux + macOS + Windows
just build-linux  # x86_64 + aarch64
just build-macos  # x86_64 + arm64
```

**Requirements:** Rust 1.75+, just task runner

## Architecture Overview

```
src/
├── main.rs              # CLI entry point (clap commands)
├── lib.rs               # Library root - re-exports Result, SerialError
├── error.rs             # Error types (SerialError, Result<T>)
├── config.rs            # Configuration management
│
├── serial_core/         # Serial port I/O
│   ├── port.rs          # PortManager, SerialConfig, PortHandle
│   ├── io_loop.rs       # Async I/O event loop
│   └── sniffer.rs       # Packet capture/monitoring
│
├── protocol/            # Protocol engine
│   ├── registry.rs      # ProtocolRegistry, ProtocolFactory
│   ├── built_in/        # Modbus, AT Command, Line protocols
│   ├── lua_ext.rs       # Custom Lua protocol support
│   └── validator.rs     # ProtocolValidator
│
├── lua/                 # LuaJIT integration
│   ├── bindings.rs      # LuaBindings - Rust→Lua API
│   ├── engine.rs        # LuaEngine runtime
│   └── executor.rs      # Script execution
│
├── task/                # Task scheduling
│   ├── queue.rs         # TaskQueue
│   └── executor.rs      # TaskExecutor
│
└── cli/                 # CLI interface
    ├── interactive.rs   # REPL shell
    ├── commands.rs      # Single commands
    └── batch.rs         # Batch script execution
```

### Key Design Patterns

1. **Protocol Trait** (`src/protocol/mod.rs`): All protocols implement `parse()`, `encode()`, `reset()`
2. **PortManager** (`src/serial_core/port.rs`): Centralized port management with UUID-based handles
3. **LuaBindings** (`src/lua/bindings.rs`): Registers Rust APIs to Lua
4. **Error Handling**: Centralized in `error.rs` using `thiserror`; all functions return `Result<T>`

## Key Conventions

- **Error handling**: Use `Result<T>` from `error.rs`
- **Async**: All I/O uses `tokio`
- **Lua integration**: Scripts executed via LuaEngine
- **Configuration**: TOML-based with fallback defaults
- **Documentation**: Avoid creating .md files unless explicitly necessary

## GUI Subproject

Tauri-based GUI in `src-tauri/` (workspace member) with React frontend in `frontend/`:

```bash
just gui-deps           # Install frontend dependencies
just gui-dev            # Start Tauri dev server
just gui-build          # Build GUI application
just gui-check          # cargo check --workspace
just gui-type-check     # TypeScript type check
just gui-fmt            # Format all code
```

## Module Dependencies

```
main.rs → cli/* → serial_core → protocol/*
                    ↓
                 lua/* → protocol/* (for custom protocols)
```
