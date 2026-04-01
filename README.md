# Serial CLI

> **Status:** ✅ Production Ready | **Version:** 0.1.0 | **Tests:** 58/58 Passing

A universal serial port CLI tool optimized for AI interaction, built with Rust.

## Quick Start

### Installation

```bash
# From source
cargo install --path .

# Or download pre-built binaries from
# https://github.com/yourusername/serial-cli/releases
```

### Basic Usage

```bash
# List available ports
serial-cli list-ports

# Interactive mode
serial-cli interactive

# Send data
serial-cli send --port=/dev/ttyUSB0 "AT+CMD"

# Run Lua script
serial-cli run script.lua
```

## Features

- **Universal** - Works with any serial device
- **AI-Optimized** - Structured JSON output for machine readability
- **Scriptable** - Embedded LuaJIT for automation
- **Cross-Platform** - Linux, macOS, Windows
- **Protocol Support** - Modbus RTU/ASCII, AT Command, Line-based, Custom Lua
- **Batch Processing** - Execute multiple scripts sequentially or in parallel

## Documentation

- **[USAGE.md](USAGE.md)** - Complete usage guide with examples
- **[DEVELOPMENT.md](DEVELOPMENT.md)** - Development setup and contributing
- **[CROSS_COMPILE.md](CROSS_COMPILE.md)** - Cross-compilation guide
- **[docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)** - Troubleshooting

## Interactive Mode Example

```bash
$ serial-cli interactive
Serial CLI Interactive Shell
Type 'help' for available commands, 'quit' to exit

serial> list
Available ports:
  /dev/ttyUSB0 - USB Serial Port

serial> open /dev/ttyUSB0
Opened port: uuid-12345678

serial> send "AT\r\n"
Sent 3 bytes

serial> recv
Received: "OK"

serial> quit
```

## Lua Scripting Example

```lua
-- Open port
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200
})

-- Send command
port:write("AT+CMD\r\n")
sleep_ms(100)

-- Read response
local response = port:read_until(string.byte("\n"))
log_info("Received: " .. response)

-- Close
port:close()
```

## Development

```bash
# Build
just build

# Run tests
just test

# Run all checks
just check

# See all commands
just --list
```

## Project Status

- ✅ Core functionality complete
- ✅ All 58 tests passing
- ✅ Cross-platform support (Linux/macOS/Windows)
- ✅ CI/CD with GitHub Actions
- ✅ Comprehensive documentation

## License

MIT OR Apache-2.0

---

**Serial CLI** - A Universal Serial Port Tool Optimized for AI Interaction
