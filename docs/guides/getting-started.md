# Getting Started

Serial CLI is a universal serial port communication tool with embedded LuaJIT scripting. It supports multiple protocols (Modbus RTU/ASCII, AT Commands, line-based, and custom Lua protocols) with structured output, optimized for both interactive use and automation.

## Installation

### Requirements

- **Rust** 1.75 or later
- **just** task runner

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install `just`:

```bash
cargo install just
```

### Building

Clone the repository and build:

```bash
git clone <repository-url>
cd serial_cli
```

```bash
# Development build
just dev

# Release build (recommended for production use)
just build
```

The binary is available at `target/debug/serial-cli` (dev) or `target/release/serial-cli` (release).

Run it directly with:

```bash
just run          # cargo run
just run <args>   # cargo run -- <args>
```

### Verification

```bash
serial-cli --help
```

## Quick Start

### List Available Ports

```bash
serial-cli list-ports
```

### Send Data to a Serial Port

```bash
serial-cli send --port /dev/ttyUSB0 "AT"
serial-cli send --port COM3 --baud 9600 "AT+CGMI"
```

### Interactive Mode

Start an interactive REPL shell for serial communication. This is the default mode when no subcommand is specified:

```bash
serial-cli interactive
serial-cli   # also starts interactive mode
```

### Run a Lua Script

```bash
serial-cli run script.lua
serial-cli run script.lua arg1 arg2
```

## Global Flags

| Flag | Description |
|------|-------------|
| `--json` | Output results as formatted JSON instead of human-readable text |
| `-v, --verbose` | Enable verbose logging output (DEBUG level) |

Examples:

```bash
serial-cli --json list-ports
serial-cli --verbose --json send --port /dev/ttyUSB0 "AT"
```

## Command Reference

### Core Commands

| Command | Description |
|---------|-------------|
| `list-ports` | List available serial ports on the system |
| `send` | Send raw data to a serial port and optionally read the response |
| `interactive` | Start an interactive REPL shell for serial communication |
| `run` | Execute a Lua script with optional arguments |

### Protocol Management

Manage built-in and custom protocols implemented in Lua.

```bash
serial-cli protocol list                    # List available protocols
serial-cli protocol list --detailed         # Show descriptions
serial-cli protocol info modbus_rtu         # Show protocol information
serial-cli protocol validate my_proto.lua   # Validate a script without loading
serial-cli protocol load my_proto.lua       # Load a custom protocol
serial-cli protocol load my_proto.lua --name myproto
serial-cli protocol unload myproto          # Unload a custom protocol
serial-cli protocol reload myproto          # Reload from disk
serial-cli protocol hot-reload enable       # Enable automatic reload on file changes
serial-cli protocol hot-reload disable
serial-cli protocol hot-reload status
```

Built-in protocols: `modbus_rtu`, `modbus_ascii`, `at_command`, `line`.

### Traffic Sniffing

Monitor and capture serial port traffic.

```bash
serial-cli sniff start --port /dev/ttyUSB0                  # Start sniffing
serial-cli sniff start --port /dev/ttyUSB0 --output cap.txt  # Save to file
serial-cli sniff start --port /dev/ttyUSB0 --max-packets 100 # Limit captures
serial-cli sniff start --port /dev/ttyUSB0 --format hex      # Hex display
serial-cli sniff stop                                       # Stop sniffing
serial-cli sniff stats                                      # Show statistics
serial-cli sniff save --path capture.log                    # Save to file
```

### Batch Execution

Run multiple Lua scripts with concurrency control.

```bash
serial-cli batch run scripts.batch                          # Run batch file
serial-cli batch run script.lua                             # Run single script
serial-cli batch run scripts.batch --concurrent 10          # Max 10 concurrent
serial-cli batch run scripts.batch --continue-on-error      # Continue on failure
serial-cli batch run scripts.batch --timeout 120            # 120s timeout per task
serial-cli batch list                                       # List available scripts
```

Batch files use a simple DSL:

```
# Comments start with #
set PORT /dev/ttyUSB0
loop 3
  script1.lua
  sleep 500
end
```

### Configuration Management

```bash
serial-cli config show               # Show current configuration
serial-cli config show --json        # Show as JSON
serial-cli config set serial.baudrate 115200
serial-cli config set logging.level debug
serial-cli config save               # Save to default location
serial-cli config save --path /etc/serial-cli.toml
serial-cli config reset              # Reset to defaults
```

### Virtual Serial Ports

Create and manage virtual serial port pairs for testing.

```bash
serial-cli virtual create                           # Auto-detect backend
serial-cli virtual create --backend pty             # PTY (Unix/macOS)
serial-cli virtual create --backend socat           # Socat-based
serial-cli virtual create --backend namedpipe       # Windows named pipes
serial-cli virtual create --monitor                 # Enable traffic monitoring
serial-cli virtual list                             # List active pairs
serial-cli virtual stop <id>                        # Stop a pair
serial-cli virtual stats <id>                       # Show statistics
```

### Performance Benchmarks

```bash
serial-cli benchmark run                            # Run all benchmarks
serial-cli benchmark run protocol                   # Run protocol benchmarks only
serial-cli benchmark run serial-io --iterations 500 # Custom iterations
serial-cli benchmark run all --output results.json  # Save results
serial-cli benchmark compare baseline.json current.json  # Compare results
serial-cli benchmark list                           # List available benchmarks
```

Benchmark categories: `serial-io`, `virtual-port`, `protocol`, `startup`, `memory`, `concurrency`, `all`.

## Development

```bash
just dev          # Development build
just build        # Release build
just test         # Run tests
just test-verbose # Run tests with full output
just check        # Format check + lint + test
just fmt          # Format code
just lint         # Clippy lint check
just run <args>   # Run with arguments
```

Cross-compilation:

```bash
just build-all    # Linux + macOS + Windows
just build-linux  # x86_64 + aarch64
just build-macos  # x86_64 + arm64
```

## Next Steps

- Read the [Architecture Guide](../dev/ARCH.md) for a deep dive into the project structure and design patterns.
- Explore protocol scripting with Lua in the `protocol/` documentation.
- Check the configuration reference for all available settings.
