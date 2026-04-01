# Serial CLI

A universal serial port CLI tool optimized for AI interaction, built with Rust.

## Features

- **Async I/O**: Built on Tokio for efficient concurrent operations
- **Multi-protocol Support**: Built-in protocols (Modbus, AT Commands, Line-based)
- **Lua Scripting**: Embedded Lua runtime for automation
- **AI-Friendly**: Structured JSON output and self-documenting design
- **Cross-platform**: Support for Windows, Linux, and macOS

## Installation

```bash
cargo install --path .
```

## Usage

### List available ports

```bash
serial-cli list-ports
```

### Send data

```bash
serial-cli send --port=COM1 "Hello, World!"
```

### Interactive mode

```bash
serial-cli interactive
```

### Run Lua script

```bash
serial-cli run script.lua
```

## Configuration

Configuration files are loaded from:
- Global: `~/.config/serial-cli/config.toml` (Linux/macOS) or `%APPDATA%\serial-cli\config.toml` (Windows)
- Project: `.serial-cli.toml` (current directory)

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run -- list-ports
```

## License

MIT OR Apache-2.0
