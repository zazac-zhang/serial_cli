<div align="center">

  ![Serial CLI](https://img.shields.io/badge/Serial%20CLI-0.4.0-blue?style=for-the-badge&logo=rust)
  [![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-green?style=for-the-badge)](LICENSE-MIT)
  [![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)
  [![GUI](https://img.shields.io/badge/GUI-Ready-success?style=for-the-badge&logo=react)](https://reactjs.org/)

  # 🚀 Serial CLI

  **A Universal Serial Port Tool with CLI & GUI - Optimized for AI Interaction**

  [Quick Start](#-quick-start) • [Documentation](#-documentation) • [Features](#-features) • [GUI Application](#-gui-application) • [Examples](#-examples) • [Lua Scripting](#-lua-scripting) • [Development](#-development)

</div>

---

## 💡 What is Serial CLI?

Serial CLI is a powerful, cross-platform serial communication tool built with Rust. It provides **both CLI and GUI interfaces**, **structured JSON output**, **embedded LuaJIT scripting**, and **support for multiple protocols** - making it perfect for both human interaction and AI/automation workflows.

**✨ Production Ready** • **🖥️ GUI Available** • **🔧 214/214 Tests Passing** • **🌍 Linux • macOS • Windows**

---

## 🚀 Quick Start

### Installation

```bash
# Install from source
cargo install --path .

# Or download pre-built binaries
# Visit: https://github.com/zazac-zhang/serial_cli/releases

# Clone repository
git clone <repository-url>
cd serial_cli

# Development build
just dev

# Release build
just build

# Run tests
just test
```

### Basic Usage

```bash
# List available serial ports
serial-cli list

# Interactive mode (open port directly)
serial-cli open /dev/ttyUSB0

# One-shot command
serial-cli exec /dev/ttyUSB0 "send AT; sleep 100; recv 64"

# Run Lua script
serial-cli run script.lua

# Data formats: text, hex (0x...), base64 (base64:...)
serial-cli exec /dev/ttyUSB0 "send 0x01020304"
serial-cli exec /dev/ttyUSB0 "send base64:SGVsbG8="
```

---

## 📖 Usage Examples

### Interactive Shell

```bash
$ serial-cli
Serial CLI Interactive Shell
Type 'help' for available commands, 'quit' to exit

serial> list
Available serial ports:
  - /dev/ttyUSB0 (UsbPort)
  - /dev/ttyACM0 (AcmPort)

serial> open /dev/ttyUSB0
Port opened successfully
Port ID: /dev/ttyUSB0-abc123

serial> send AT
Sent 2 bytes

serial> recv 64
Received (4 bytes): OK

serial> quit
```

---

## ✨ Features

<div align="center">

| 🎯 **Universal** | 🤖 **AI-Optimized** | ⚡ **Scriptable** | 🌍 **Cross-Platform** |
|:---:|:---:|:---:|:---:|
| Works with any serial device | Structured JSON output | Embedded LuaJIT runtime | Linux • macOS • Windows |

| 📡 **Protocols** | 🔄 **Batch Mode** | 🔍 **Sniff Sessions** | 🖥️ **GUI Available** |
|:---:|:---:|:---:|:---:|
| Modbus • AT Commands • Custom | Variables, loops, error reporting | Start/stop/stats/save | Tauri-based GUI (NEW!) |

</div>

### Core Capabilities

- **🔌 Serial Port Management** - List, open, configure, and manage serial ports
- **📜 Lua Scripting** - Automate tasks with embedded LuaJIT (high-performance)
- **📡 Protocol Support** - Built-in Modbus RTU/ASCII, AT Commands, line-based, and **custom Lua protocols**
- **🎨 Custom Protocols** - Load custom protocols from Lua scripts with hot-reload support
- **🤖 AI-Friendly** - JSON output mode for easy integration with AI systems
- **🔄 Batch Processing** - Execute multiple scripts with variable substitution, loops, and per-script error reporting
- **🔍 Sniff Sessions** - Start/stop/stats/save serial traffic with background daemon and session management
- **🖥️ GUI Application** - **NEW!** Modern Tauri-based GUI with:
  - Cyber-industrial aesthetic design
  - Real-time data monitoring
  - Monaco script editor
  - Protocol management
  - Multi-format data export (TXT/CSV/JSON)
  - System notifications
  - Complete keyboard shortcuts
  - Data persistence
- **🔌 Virtual Serial Ports** - **NEW!** Pluggable backend architecture:
  - **PTY Backend** (Unix/macOS) - POSIX pseudo-terminals
  - **NamedPipe Backend** (Windows) - Windows named pipes
  - **Socat Backend** (Cross-platform) - Socat-based virtual ports
  - Platform auto-detection (defaults to best backend for your OS)
  - Runtime backend selection via CLI flag or config file

---

### One-Shot Commands

```bash
# Send command and receive response
serial-cli exec /dev/ttyUSB0 "send AT; sleep 100; recv 64"

# With custom baud rate
serial-cli exec /dev/ttyUSB0 --baudrate 9600 "send data"

# With protocol
serial-cli exec /dev/ttyUSB0 --protocol modbus_rtu "send 0x010300000001"

# Hex data
serial-cli exec /dev/ttyUSB0 "send 0x01020304"

# Base64 data
serial-cli exec /dev/ttyUSB0 "send base64:SGVsbG8="
```

#### Data Sniffing — Session Management

```bash
# Start sniffing on a port (spawns background daemon)
serial-cli sniff start -p /dev/ttyUSB0 --output capture.log

# Check sniff session statistics
serial-cli sniff stats

# Save captured packets to a file
serial-cli sniff save -p backup.log

# Stop the active sniff session
serial-cli sniff stop
```

### Batch Processing — Variables & Loops

```bash
# Run a single Lua script
serial-cli batch run script.lua

# Run a batch file with variable substitution and loops
serial-cli batch run tasks.batch

# List available batch scripts
serial-cli batch list
```

**Batch file example** (`tasks.batch`):
```bash
# Set variables (also reads from environment)
set PORT /dev/ttyUSB0
set DEVICE modbus

# Run scripts with variable substitution
scripts/${DEVICE}/init.lua
scripts/${DEVICE}/read.lua

# Loop with sleep
loop 3
  scripts/${DEVICE}/poll.lua
  sleep 500
end
```

### Virtual Serial Ports — Testing & Development

```bash
# Create virtual port pair (auto-detects best backend)
serial-cli virtual create

# Create with specific backend
serial-cli virtual create --backend pty          # Unix/macOS
serial-cli virtual create --backend namedpipe   # Windows
serial-cli virtual create --backend socat       # Cross-platform (requires socat)

# Create with monitoring enabled
serial-cli virtual create --monitor --max-packets 1000

# List active virtual pairs
serial-cli virtual list

# Show statistics for a pair
serial-cli virtual stats <id>

# Stop a virtual pair
serial-cli virtual stop <id>
```

**Virtual ports are perfect for:**
- Testing serial applications without hardware
- Development and debugging
- CI/CD pipeline automation
- Protocol development and validation

**Backend Selection:**
- **Auto** (recommended): Automatically selects the best backend for your platform
- **PTY**: Best performance on Unix/macOS
- **NamedPipe**: Native Windows implementation
- **Socat**: Cross-platform alternative (requires `socat` installation)

**Set default backend in config:**
```bash
serial-cli config set virtual.backend socat
```

---

## Lua Scripting - Modbus RTU

```lua
-- modbus_read.lua
local port_name = "/dev/ttyUSB0"
local slave_id = 1
local start_addr = 0
local reg_count = 10

-- Open port with Modbus settings
local port = serial_open(port_name, {
  baudrate = 19200,
  databits = 8,
  parity = "even",
  stopbits = 1
})

-- Build Modbus request (function 0x03 = Read Holding Registers)
local request = string.char(
  slave_id, 0x03,
  (start_addr >> 8) & 0xFF, start_addr & 0xFF,
  (reg_count >> 8) & 0xFF, reg_count & 0xFF
)

-- Calculate CRC
local crc = 0xFFFF
for i = 1, #request do
  crc = crc ~ string.byte(request, i)
  for j = 1, 8 do
    if (crc & 0x0001) ~= 0 then
      crc = (crc >> 1) ~ 0xA001
    else
      crc = crc >> 1
    end
  end
end
request = request .. string.char(crc & 0xFF, (crc >> 8) & 0xFF)

-- Send and receive
serial_send(port, request)
sleep(100)
local response = serial_recv(port, 256)

print("Response: " .. hex_encode(response))
serial_close(port)
```

**Run:** `serial-cli run modbus_read.lua`

### Data Logging

```lua
-- data_logger.lua
local port = serial_open("/dev/ttyUSB0", {baudrate = 115200})
local file = io.open("log.txt", "w")

file:write("# Data log started at " .. os.date() .. "\n")

for i = 1, 100 do
  local data = serial_recv(port, 1024)
  if #data > 0 then
    file:write(data)
    file:flush()
    print("Received " .. #data .. " bytes")
  end
  sleep(50)
end

file:close()
serial_close(port)
```

**Run:** `serial-cli run data_logger.lua`

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

# Start GUI development server
just gui-dev

# Build GUI application
just gui-build

# Type check frontend
just gui-type-check

# Format all code (Rust + TypeScript)
just gui-fmt

# Check Rust + TypeScript code
just gui-check
```

**GUI Features (v0.2.0)**:
- ✅ Serial port management with configuration UI
- ✅ Real-time data monitoring with RX/TX distinction
- ✅ Lua script editor with Monaco Editor
- ✅ Protocol loading and validation
- ✅ Settings management with persistence
- ✅ Data export (TXT/CSV/JSON)
- ✅ System notifications
- ✅ Command palette (⌘K)
- ✅ Keyboard shortcuts
- ✅ Data persistence (localStorage)

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
│   └── GUIDE.md            # GUI application guide
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

### Virtual Port Backend Dependencies

**Socat Backend** (Cross-platform):
```bash
# Linux (Debian/Ubuntu)
sudo apt-get install socat

# macOS
brew install socat

# Windows
# Download from: http://www.dest-unreach.org/socat/
# Or use WSL on Windows
```

**Note:** The PTY and NamedPipe backends have no external dependencies and work out of the box on their respective platforms.

---

## 📚 Documentation

| Document | Description |
|:---|:---|
| **[DEVELOPMENT.md](DEVELOPMENT.md)** | Development guide for contributors |
| **[docs/GUIDE.md](docs/GUIDE.md)** | GUI application user guide |

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
