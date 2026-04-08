<div align="center">

  ![Serial CLI](https://img.shields.io/badge/Serial%20CLI-0.1.0-blue?style=for-the-badge&logo=rust)
  [![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-green?style=for-the-badge)](LICENSE-MIT)
  [![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)

  # 🚀 Serial CLI

  **A Universal Serial Port CLI Tool Optimized for AI Interaction**

  [Quick Start](#-quick-start) • [Features](#-features) • [Examples](#-examples) • [Lua Scripting](#-lua-scripting) • [Development](#-development)

</div>

---

## 💡 What is Serial CLI?

Serial CLI is a powerful, cross-platform serial communication tool built with Rust. It provides **structured JSON output**, **embedded LuaJIT scripting**, and **support for multiple protocols** - making it perfect for both human interaction and AI/automation workflows.

**✨ Production Ready** • **🔧 58/58 Tests Passing** • **🌍 Linux • macOS • Windows**

---

## 🚀 Quick Start

### Installation

```bash
# Install from source
cargo install --path .

# Or download pre-built binaries
# Visit: https://github.com/zazac-zhang/serial_cli/releases
```

### Basic Usage

```bash
# List available serial ports
serial-cli list-ports

# Start interactive mode
serial-cli interactive

# Send data to a port
serial-cli send --port=/dev/ttyUSB0 "AT+CMD"

# Run a Lua script
serial-cli run script.lua --port=/dev/ttyUSB0
```

---

## ✨ Features

<div align="center">

| 🎯 **Universal** | 🤖 **AI-Optimized** | ⚡ **Scriptable** | 🌍 **Cross-Platform** |
|:---:|:---:|:---:|:---:|
| Works with any serial device | Structured JSON output | Embedded LuaJIT runtime | Linux • macOS • Windows |

| 📡 **Protocols** | 🔄 **Batch Mode** | 🖥️ **GUI Available** | 🧪 **Well-Tested** |
|:---:|:---:|:---:|:---:|
| Modbus • AT Commands • Custom | Sequential & Parallel execution | Tauri-based GUI | 58 passing tests |

</div>

### Core Capabilities

- **🔌 Serial Port Management** - List, open, configure, and manage serial ports
- **📜 Lua Scripting** - Automate tasks with embedded LuaJIT (high-performance)
- **📡 Protocol Support** - Built-in Modbus RTU/ASCII, AT Commands, line-based, and **custom Lua protocols**
- **🎨 Custom Protocols** - Load custom protocols from Lua scripts with hot-reload support
- **🤖 AI-Friendly** - JSON output mode for easy integration with AI systems
- **🔄 Batch Processing** - Execute multiple scripts sequentially or in parallel
- **🖥️ Interactive Shell** - Powerful REPL shell with command history and auto-completion
- **🎛️ GUI Application** - Tauri-based cross-platform GUI (optional)

---

## 📖 Examples

### Interactive Mode

```bash
$ serial-cli interactive
Serial CLI Interactive Shell
Type 'help' for available commands, 'quit' to exit

serial> list
Available ports:
  /dev/ttyUSB0 - USB Serial Port

serial> open /dev/ttyUSB0 --baudrate=115200
✓ Opened port: uuid-12345678

serial> send "AT\r\n"
✓ Sent 3 bytes

serial> recv
Received: "OK"

serial> quit
```

### Lua Scripting - Modbus RTU

```lua
local modbus = require('serial.protocols.modbus')

-- Open port with Modbus-friendly settings
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 9600,
    parity = "even",
    stop_bits = 1
})

-- Create Modbus handler
local client = modbus.new(port, {
    device_id = 1,
    timeout = 1000
})

-- Read holding registers
local registers = client:read_holding_registers(0x0000, 10)
print(string.format("Read %d registers", #registers))

-- Write single register
client:write_single_register(0x0001, 0x0042)

client:close()
port:close()
```

### Lua Scripting - Custom Protocol

```lua
-- Custom protocol implementation
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200,
    timeout = 1000
})

-- Build custom frame
local function build_frame(cmd, data)
    local frame = string.char(0xAA, cmd, #data) .. data
    local crc = calculate_crc(frame)
    return frame .. string.char(crc, 0x55)
end

-- Send command
port:write(build_frame(0x01, "ping"))

-- Parse response
local response = port:read(256)
if validate_frame(response) then
    log_info("Valid response received")
end

port:close()
```

---

## 🔧 Lua Scripting API

Serial CLI includes an embedded **LuaJIT** runtime for powerful automation:

### Serial Port Functions

```lua
-- Open serial port
local port = serial.open(path, {
    baudrate = 115200,      -- Baud rate (default: 9600)
    timeout = 1000,         -- Read timeout in ms (default: 1000)
    data_bits = 8,          -- Data bits: 7 or 8 (default: 8)
    parity = "none",        -- Parity: "none", "odd", "even" (default: "none")
    stop_bits = 1,          -- Stop bits: 1 or 2 (default: 1)
    flow_control = "none"   -- Flow control: "none", "hardware", "software"
})

-- Write data
port:write("Hello, World!\r\n")

-- Read data
local data = port:read(256)
local line = port:read_until("\n")

-- Close port
port:close()
```

### Protocol Tools

```lua
-- Modbus RTU/ASCII
local modbus = serial.protocols.modbus.new(port, {
    device_id = 1,
    timeout = 1000
})

modbus:read_holding_registers(addr, count)
modbus:read_input_registers(addr, count)
modbus:write_single_register(addr, value)
modbus:write_multiple_registers(addr, values)
```

### Utility Functions

```lua
-- Logging
log_info("Information message")
log_warn("Warning message")
log_error("Error message")

-- Utilities
sleep_ms(1000)
local hex = bytes_to_hex(data)
local bytes = hex_to_bytes("48656c6c6f")
```

### Custom Protocol Extension

Serial CLI supports loading custom protocols from Lua scripts:

```lua
-- Load custom protocol
local ok, err = protocol_load("/path/to/my_protocol.lua")
if ok then
    local encoded = protocol_encode("my_custom_protocol", "data")
    local decoded = protocol_decode("my_custom_protocol", encoded)
end
```

**Protocol Requirements:**
- `on_frame(data)` - Parse incoming data
- `on_encode(data)` - Encode outgoing data
- `on_reset()` - Reset protocol state (optional)

See `examples/` directory for complete protocol examples.

---

## 🛠️ Development

### Prerequisites

```bash
# Rust 1.75+
rustup update stable

# Just task runner (recommended)
cargo install just

# Platform dependencies
# Linux:
sudo apt-get install build-essential libudev-dev

# macOS:
xcode-select --install
```

### Build Commands

```bash
# Development build
just dev          # cargo build

# Release build
just build        # cargo build --release

# Run application
just run <args>   # cargo run -- <args>

# Run all checks (fmt + lint + test)
just check
```

### Testing

```bash
# Run all tests
just test

# Run specific test
just test <test_name>

# Run tests with output
just test-verbose
```

### Code Quality

```bash
# Format code
just fmt

# Check formatting
just fmt-check

# Run linter
just lint

# Cross-compilation
just build-all    # All platforms
just build-linux  # Linux (x86_64 + aarch64)
just build-macos  # macOS (x86_64 + arm64)
just build-windows # Windows (requires cross)
```

### GUI Development

```bash
# Install GUI dependencies
just gui-deps

# Start GUI development
just gui-dev

# Build GUI application
just gui-build

# Type check frontend
just gui-type-check
```

### Project Structure

```
serial_cli/
├── src/                    # Rust library (core functionality)
│   ├── main.rs             # CLI entry point
│   ├── lib.rs              # Library root
│   ├── error.rs            # Error types
│   ├── config.rs           # Configuration
│   ├── serial_core/        # Serial port I/O
│   ├── protocol/           # Protocol engine
│   ├── lua/                # LuaJIT integration
│   ├── task/               # Task scheduling
│   └── cli/                # CLI interface
├── src-tauri/              # Tauri application (GUI backend)
│   ├── src/                # Tauri-specific code
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── frontend/               # React frontend (GUI)
│   ├── src/                # React source
│   ├── components/
│   ├── index.html
│   └── package.json
├── examples/               # Lua script examples
├── tests/                  # Integration tests
├── docs/                   # Documentation
├── justfile                # Build commands
├── Cargo.toml              # Package config
└── README.md               # This file
```

---

## 🔍 Troubleshooting

### Common Issues

#### 1. Permission Denied

**Error:** `Permission denied for port '/dev/ttyUSB0'`

**Solution (Linux):**
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER
# Re-login or execute
newgrp dialout
```

**Solution (Windows):**
- Run as Administrator
- Close other applications using the port

#### 2. Port Not Found

**Error:** `Port '/dev/ttyUSB0' not found`

**Solution:**
- Use `serial-cli list-ports` to verify available ports
- Check USB connection and cables
- Windows: Check COM port in Device Manager

#### 3. Timeout Error

**Error:** `Operation timeout`

**Solution:**
- Increase timeout: `timeout = 5000`
- Verify baudrate matches device
- Check device is responding

#### 4. Port In Use

**Error:** `Port 'COM1' is already in use`

**Solution:**
- Close PuTTY, Tera Term, Arduino IDE, etc.
- Disable/enable port in Device Manager

#### 5. Lua Script Error

**Error:** `Runtime error in script.lua`

**Solution:**
- Use `--verbose` for detailed error
- Verify Lua syntax
- Check API calls

### Debug Mode

```bash
# Enable verbose logging
serial-cli --verbose list-ports
serial-cli --verbose run script.lua

# Set log level
RUST_LOG=debug serial-cli list-ports
RUST_LOG=trace serial-cli list-ports
```

### Platform-Specific

**Linux:**
```bash
# Install dependencies
sudo apt-get install build-essential libudev-dev
```

**macOS:**
```bash
# Install Xcode tools
xcode-select --install
```

**Windows:**
- Install drivers for FTDI, CP210x, CH340 USB-to-serial adapters
- Arduino IDE includes common drivers
- Install Visual Studio Build Tools for development

---

## 📚 Documentation

| Document | Description |
|:---|:---|
| **[DEVELOPMENT.md](DEVELOPMENT.md)** | Development guide and contribution flow |
| **[docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)** | Detailed troubleshooting |

---

## 🤝 Contributing

Contributions are welcome! Please read [DEVELOPMENT.md](DEVELOPMENT.md) for details on our code of conduct, development setup, and submission process.

---

## 📝 License

Dual-licensed under:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

---

<div align="center">

**Built with ❤️ and Rust**

[GitHub](https://github.com/zazac-zhang/serial_cli) • [Report Issues](https://github.com/zazac-zhang/serial_cli/issues) • [Releases](https://github.com/zazac-zhang/serial_cli/releases)

</div>
