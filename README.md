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
- **[docs/windows.md](docs/windows.md)** - Windows platform guide
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

## Lua Scripting

Serial CLI includes an embedded LuaJIT runtime for powerful automation and custom protocol implementation.

### Quick Start

```bash
# Run a script
serial-cli run examples/raw_echo.lua --port=/dev/ttyUSB0

# Pass arguments to script
serial-cli run examples/modbus_with_tools.lua /dev/ttyUSB0 9600 1
```

### Example Scripts

#### 1. Raw I/O (`examples/raw_echo.lua`)
Simple serial communication without protocol handling:

```lua
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200,
    timeout = 1000
})

-- Send data
port:write("Hello, World!\r\n")

-- Read response
local response = port:read(256)
log_info("Received: " .. response)

port:close()
```

#### 2. Modbus RTU (`examples/modbus_with_tools.lua`)
Using built-in Modbus protocol tools:

```lua
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 9600,
    parity = "even"
})

local modbus = serial.protocols.modbus.new(port, {
    device_id = 1,
    timeout = 1000
})

-- Read holding registers
local registers = modbus:read_holding_registers(0x0000, 10)

-- Write single register
modbus:write_single_register(0x0001, 0x0042)

modbus:close()
port:close()
```

#### 3. Custom Protocol (`examples/custom_protocol.lua`)
Implement your own protocol:

```lua
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200
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
-- ... parse and validate ...

port:close()
```

### Lua API Reference

#### Serial Port Functions

- `serial.open(path, config) -> port` - Open serial port
  - `config.baudrate` - Baud rate (default: 9600)
  - `config.timeout` - Read timeout in ms (default: 1000)
  - `config.data_bits` - Data bits: 7 or 8 (default: 8)
  - `config.parity` - Parity: "none", "odd", "even" (default: "none")
  - `config.stop_bits` - Stop bits: 1 or 2 (default: 1)

- `port:write(data) -> bytes_written` - Write data to port
- `port:read(max_bytes) -> data` - Read data from port
- `port:read_until(delimiter) -> data` - Read until delimiter byte
- `port:close()` - Close the port

#### Protocol Tools

- `serial.protocols.modbus.new(port, config) -> modbus` - Create Modbus handler
  - `modbus:read_holding_registers(addr, count) -> values`
  - `modbus:read_input_registers(addr, count) -> values`
  - `modbus:write_single_register(addr, value) -> success`
  - `modbus:write_multiple_registers(addr, values) -> success`
  - `modbus:close()`

#### Utility Functions

- `log_info(message)` - Log info message
- `log_warn(message)` - Log warning message
- `log_error(message)` - Log error message
- `sleep_ms(milliseconds)` - Sleep for specified time
- `bytes_to_hex(bytes) -> hex_string` - Convert bytes to hex string
- `hex_to_bytes(hex_string) -> bytes` - Convert hex string to bytes
- `string_to_bytes(str) -> bytes` - Convert string to byte array
- `bytes_to_string(bytes) -> str` - Convert byte array to string

For complete documentation, see [USAGE.md](USAGE.md#lua-scripting).

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
