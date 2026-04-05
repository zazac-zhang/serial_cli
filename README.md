<div align="center">

  ![Serial CLI](https://img.shields.io/badge/Serial%20CLI-0.1.0-blue?style=for-the-badge&logo=rust)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/zazac-zhang/serial_cli/ci.yml?branch=master&style=for-the-badge&logo=github)](https://github.com/zazac-zhang/serial_cli/actions)
  [![License](https://img.shields.io/badge/License-MIT%20%2F%20Apache--2.0-green?style=for-the-badge)](LICENSE-MIT)
  [![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org)

  # 🚀 Serial CLI

  **A Universal Serial Port CLI Tool Optimized for AI Interaction**

  [Quick Start](#-quick-start) • [Features](#-features) • [Examples](#-examples) • [Lua Scripting](#-lua-scripting) • [Docs](#-documentation)

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
cargo install serial-cli

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

| 📡 **Protocols** | 🔄 **Batch Mode** | 🛠️ **Interactive** | 🧪 **Well-Tested** |
|:---:|:---:|:---:|:---:|
| Modbus • AT Commands • Custom | Sequential & Parallel execution | REPL shell | 58 passing tests |

</div>

### Core Capabilities

- **🔌 Serial Port Management** - List, open, configure, and manage serial ports
- **📜 Lua Scripting** - Automate tasks with embedded LuaJIT (high-performance)
- **📡 Protocol Support** - Built-in Modbus RTU/ASCII, AT Commands, line-based, and **custom Lua protocols**
- **🎨 Custom Protocols** - Load custom protocols from Lua scripts with hot-reload support
- **🤖 AI-Friendly** - JSON output mode for easy integration with AI systems
- **🔄 Batch Processing** - Execute multiple scripts sequentially or in parallel
- **🖥️ Interactive Shell** - Powerful REPL with command history and auto-completion

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

📚 **Complete Lua API:** [USAGE.md](USAGE.md#lua-scripting)

---

## 📚 Documentation

| Document | Description |
|:---|:---|
| **[USAGE.md](USAGE.md)** | Complete usage guide with all commands and options |
| **[DEVELOPMENT.md](DEVELOPMENT.md)** | Development setup, architecture, and contributing |
| **[CROSS_COMPILE.md](CROSS_COMPILE.md)** | Cross-compilation guide for different platforms |
| **[docs/windows.md](docs/windows.md)** | Windows-specific configuration and troubleshooting |
| **[docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)** | Common issues and solutions |

---

## 🛠️ Development

```bash
# Build the project
just build

# Run tests
just test

# Run all checks (tests, lint, fmt)
just check

# List all available commands
just --list
```

**Requirements:** Rust 1.75+, just task runner

---

## 📊 Project Status

<div align="center">

✅ **Core functionality complete** • ✅ **All 58 tests passing** • ✅ **Cross-platform support** • ✅ **CI/CD configured**

</div>

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

## 🎨 Custom Protocol Extension

Serial CLI now supports loading custom protocols from Lua scripts! This enables you to implement proprietary or industry-specific protocols without modifying the core codebase.

### Creating a Custom Protocol

Create a Lua file with your protocol implementation:

```lua
-- Protocol: my_custom_protocol
-- This implements a simple frame-based protocol

function on_frame(data)
    -- Process incoming data
    -- Return parsed data or nil for invalid frames
    return data
end

function on_encode(data)
    -- Encode outgoing data
    -- Return encoded data
    return data
end

function on_reset()
    -- Optional: Called when protocol state resets
end
```

### Loading Custom Protocols

**Via Lua Script:**
```lua
local ok, err = protocol_load("/path/to/my_protocol.lua")
if ok then
    local encoded = protocol_encode("my_custom_protocol", "data")
    local decoded = protocol_decode("my_custom_protocol", encoded)
end
```

**Via Validation API:**
```lua
local ok, err = protocol_validate("/path/to/my_protocol.lua")
-- Validates syntax and required functions before loading
```

### Protocol Requirements

Every custom protocol must implement:
- `on_frame(data)` - Parse incoming data (return `data` for passthrough, `nil` for error)
- `on_encode(data)` - Encode outgoing data
- `on_reset()` (optional) - Reset protocol state

### Example: Checksum Protocol

See `examples/checksum_protocol.lua` for a complete example with frame validation and checksum calculation.

### Protocol Management API

- `protocol_load(path)` - Load protocol from file
- `protocol_unload(name)` - Unload a protocol
- `protocol_reload(name)` - Reload protocol from file
- `protocol_list()` - List all protocols
- `protocol_info(name)` - Get protocol information
- `protocol_validate(path)` - Validate without loading

For more examples, see:
- `examples/protocol_extension_demo.lua` - Complete API demo
- `examples/custom_protocol.lua` - Binary protocol with CRC

