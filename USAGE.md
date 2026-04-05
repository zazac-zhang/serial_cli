# Serial CLI - Usage Guide

Complete usage documentation for Serial CLI, a universal serial port tool optimized for AI interaction.

## Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Command Reference](#command-reference)
- [Interactive Mode](#interactive-mode)
- [Lua Scripting API](#lua-scripting-api)
- [Protocol Configuration](#protocol-configuration)
- [Batch Processing](#batch-processing)
- [Troubleshooting](#troubleshooting)
- [Examples](#examples)

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/serial-cli.git
cd serial-cli

# Build release version
cargo build --release

# Install locally
cargo install --path .
```

### Using Pre-built Binaries

Download the appropriate binary for your platform from the [Releases](https://github.com/yourusername/serial-cli/releases) page.

- **Linux**: `serial-cli-linux-x86_64` or `serial-cli-linux-aarch64`
- **macOS**: `serial-cli-macos-x86_64` or `serial-cli-macos-arm64`
- **Windows**: `serial-cli-windows-x86_64.exe`

Make the binary executable (Linux/macOS):
```bash
chmod +x serial-cli-*
mv serial-cli-* /usr/local/bin/serial-cli
```

### Requirements

- Rust 1.70+ (if building from source)
- Serial port permissions (Linux: `sudo usermod -a -G dialout $USER`)
- For cross-compilation: Docker (for `cross` tool)

## Quick Start

### List Available Ports

```bash
serial-cli list-ports
```

Output (JSON):
```json
[
  {
    "port_name": "/dev/ttyUSB0",
    "port_type": "UsbPort"
  },
  {
    "port_name": "/dev/ttyACM0",
    "port_type": "UsbPort"
  }
]
```

### Send Data

```bash
# Send AT command
serial-cli send --port=/dev/ttyUSB0 "AT+CMD"

# With JSON output
serial-cli --json send --port=/dev/ttyUSB0 "AT+CMD"
```

### Interactive Mode

```bash
serial-cli interactive
```

```
Serial CLI Interactive Shell
Type 'help' for available commands, 'quit' to exit

serial> list
serial> open /dev/ttyUSB0
serial> send "AT+CMD"
serial> recv
serial> close
serial> quit
```

### Run Lua Script

```bash
serial-cli run script.lua
```

## Command Reference

### Global Options

- `--json`: Enable JSON output for all commands
- `-v, --verbose`: Enable verbose logging

### Available Commands

#### `list-ports`

List all available serial ports on the system.

```bash
serial-cli list-ports
```

**Output:**
- JSON array of port objects
- Each port has `port_name` and `port_type` fields

#### `interactive`

Start interactive shell for serial communication.

```bash
serial-cli interactive
```

See [Interactive Mode](#interactive-mode) for details.

#### `send`

Send data to a serial port.

```bash
serial-cli send --port=<PORT> <DATA>
```

**Arguments:**
- `--port, -p`: Port name (e.g., `/dev/ttyUSB0`, `COM1`)
- `data`: Data string to send

**Example:**
```bash
serial-cli send --port=/dev/ttyUSB0 "AT+CMD\r\n"
```

#### `recv`

Receive data from a serial port (available in interactive mode).

```bash
serial> recv [n]
```

**Arguments:**
- `n`: Optional number of bytes to receive (default: 64)

#### `status`

Show status of current serial port (interactive mode).

```bash
serial> status
```

#### `run`

Execute a Lua script.

```bash
serial-cli run <SCRIPT_FILE>
```

**Example:**
```bash
serial-cli run examples/basic_io.lua
```

#### `batch`

Execute multiple Lua scripts in batch.

```bash
# Sequential execution
serial-cli batch script1.lua script2.lua script3.lua

# Parallel execution
serial-cli batch --parallel script1.lua script2.lua script3.lua
```

## Interactive Mode

Interactive mode provides a REPL shell for serial communication.

### Starting Interactive Mode

```bash
serial-cli interactive
```

### Available Commands

#### `help`
Show available commands.

```bash
serial> help
```

#### `list`
List available serial ports.

```bash
serial> list
```

#### `open <port>`
Open a serial port with default configuration (115200 8N1).

```bash
serial> open /dev/ttyUSB0
```

#### `close [port_id]`
Close the current port or specified port ID.

```bash
serial> close
```

#### `send <data>`
Send data to the current port.

```bash
serial> send "AT+CMD\r\n"
serial> send "Hello World"
```

#### `recv [n]`
Receive data from current port (default: 64 bytes).

```bash
serial> recv
serial> recv 128
```

#### `status`
Display current port status and configuration.

```bash
serial> status
```

#### `protocol <name>`
Set protocol for current port.

```bash
serial> protocol modbus-rtu
serial> protocol at-command
serial> protocol line
serial> protocol custom-lua
```

#### `quit` or `exit`
Exit interactive mode.

```bash
serial> quit
```

### Example Session

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

serial> status
Port ID: uuid-12345678
Port: /dev/ttyUSB0
Baud rate: 115200
Data bits: 8
Stop bits: 1
Parity: None

serial> close
Closed port: uuid-12345678

serial> quit
Goodbye!
```

## Lua Scripting API

Serial CLI provides a powerful Lua scripting API with LuaJIT for high-performance automation.

### Standard Library Functions

#### Logging Functions

```lua
-- Info level
log_info("Starting script")

-- Debug level
log_debug("Variable value: " .. value)

-- Warning level
log_warn("Unexpected response")

-- Error level
log_error("Operation failed")
```

#### Utility Functions

```lua
-- Sleep for milliseconds
sleep_ms(100)

-- JSON encoding (basic types)
local json_str = json_encode({status = "ok", value = 42})

-- JSON decoding (placeholder - not fully implemented)
local data = json_decode(json_str)
```

#### String Utilities

```lua
-- Convert string to hexadecimal
local hex = string_to_hex("ABC")
-- Result: "414243"

-- Convert hexadecimal to string
local str = hex_to_string("414243")
-- Result: "ABC"
```

#### Time Utilities

```lua
-- Get current timestamp in milliseconds
local ts = timestamp_ms()
```

#### Serial Port API

```lua
-- Open a serial port
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200,
    databits = 8,
    stopbits = 1,
    parity = "none"  -- "none", "odd", "even"
})

-- Send data
port:write("AT+CMD\r\n")

-- Read until specific byte/sequence
local response = port:read_until(string.byte("\n"))

-- Read exact number of bytes
local data = port:read_exact(10)

-- Read with timeout (milliseconds)
local data = port:read(100)  -- 100ms timeout

-- Close port
port:close()
```

### Complete Script Example

```lua
-- examples/comprehensive_test.lua

-- Log script start
log_info("Starting comprehensive test")

-- Open serial port
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200,
    databits = 8,
    stopbits = 1,
    parity = "none"
})

log_info("Port opened successfully")

-- Send AT command
port:write("AT\r\n")
sleep_ms(100)

-- Read response
local response = port:read_until(string.byte("\n"))
log_info("Received: " .. response)

-- Convert to hex for analysis
local hex = string_to_hex(response)
log_debug("Hex representation: " .. hex)

-- Create structured output
local result = {
    status = "success",
    response = response,
    hex = hex,
    timestamp = timestamp_ms()
}

-- Encode and print
local json_str = json_encode(result)
print(json_str)

-- Clean up
port:close()
log_info("Test complete")
```

### Custom Protocol Example

```lua
-- examples/custom_protocol.lua

-- Custom protocol with Lua callbacks
local protocol = {
    -- Encode function: called when sending data
    encode = function(data)
        log_info("Encoding: " .. data)

        -- Add header
        local header = string.char(0xAA, 0x55)
        local length = string.char(#data)

        -- Calculate checksum (simple XOR)
        local checksum = 0
        for i = 1, #data do
            checksum = checksum ~ string.byte(data, i)
        end

        return header .. length .. data .. string.char(checksum)
    end,

    -- Decode function: called when receiving data
    decode = function(raw_data)
        log_info("Decoding " .. #raw_data .. " bytes")

        -- Verify header
        if string.sub(raw_data, 1, 2) ~= string.char(0xAA, 0x55) then
            log_error("Invalid header")
            return nil
        end

        -- Extract length
        local length = string.byte(raw_data, 3)
        log_debug("Payload length: " .. length)

        -- Extract payload
        local payload = string.sub(raw_data, 4, 4 + length - 1)

        -- Verify checksum
        local checksum = string.byte(raw_data, 4 + length)
        local calculated = 0
        for i = 1, #payload do
            calculated = calculated ~ string.byte(payload, i)
        end

        if checksum ~= calculated then
            log_error("Checksum mismatch")
            return nil
        end

        return payload
    end
}

-- Use the protocol
local port = serial.open("/dev/ttyUSB0", {})

-- Register protocol (if API supports it)
-- port:set_protocol(protocol)

-- Send data using protocol
local encoded = protocol.encode("Hello")
port:write(encoded)

-- Receive and decode
local raw = port:read(1000)
local decoded = protocol.decode(raw)

if decoded then
    log_info("Decoded: " .. decoded)
end

port:close()
```

## Protocol Configuration

Serial CLI supports multiple built-in protocols and custom Lua protocols.

### Built-in Protocols

#### Modbus RTU

Modbus RTU protocol with CRC16 checksum.

```bash
serial> protocol modbus-rtu
```

**Features:**
- CRC16 checksum calculation
- 3.5 character time gap detection
- Standard Modbus frame format

#### Modbus ASCII

Modbus ASCII protocol with LRC checksum.

```bash
serial> protocol modbus-ascii
```

**Features:**
- LRC checksum calculation
- ASCII encoding (hex)
- CRLF line endings
- Start (: ) and end (CR/LF) markers

#### AT Command

AT command protocol with standard formatting.

```bash
serial> protocol at-command
```

**Features:**
- Automatic CRLF appending
- Response parsing (OK/ERROR)
- Timeout handling

#### Line-based

Simple line-based protocol.

```bash
serial> protocol line
```

**Features:**
- Lines terminated by newline
- Text-based communication
- Simple and reliable

### Custom Lua Protocol

Create custom protocols using Lua callbacks.

```lua
-- Define protocol
local my_protocol = {
    encode = function(data)
        -- Custom encoding logic
        return encoded_data
    end,

    decode = function(raw_data)
        -- Custom decoding logic
        return decoded_data
    end
}

-- Register and use
-- (API depends on implementation)
```

## Batch Processing

Execute multiple Lua scripts sequentially or in parallel.

### Sequential Execution

Scripts run one after another.

```bash
serial-cli batch script1.lua script2.lua script3.lua
```

**Use cases:**
- Dependent operations
- Resource-constrained environments
- Debugging

### Parallel Execution

Scripts run concurrently.

```bash
serial-cli batch --parallel script1.lua script2.lua script3.lua
```

**Use cases:**
- Independent operations
- Multi-port testing
- Performance optimization

### Batch Script Example

**script1.lua** - Initialize device:
```lua
log_info("Initializing device")
local port = serial.open("/dev/ttyUSB0", {})
port:write("AT+INIT\r\n")
sleep_ms(500)
port:close()
```

**script2.lua** - Configure device:
```lua
log_info("Configuring device")
local port = serial.open("/dev/ttyUSB0", {})
port:write("AT+CONFIG=1\r\n")
sleep_ms(300)
port:close()
```

**script3.lua** - Test device:
```lua
log_info("Testing device")
local port = serial.open("/dev/ttyUSB0", {})
port:write("AT+TEST\r\n")
local response = port:read(1000)
log_info("Response: " .. response)
port:close()
```

Run all:
```bash
serial-cli batch script1.lua script2.lua script3.lua
```

## Troubleshooting

### Permission Issues (Linux)

**Problem:** `Permission denied` when accessing serial port.

**Solution:**
```bash
# Add user to dialout group
sudo usermod -a -G dialout $USER

# Log out and log back in, or use:
newgrp dialout

# Verify groups
groups
```

### Port Not Found

**Problem:** Port not listed in `list-ports`.

**Solutions:**
1. Check physical connection
2. Verify driver installation (`lsmod | grep usbserial`)
3. Check dmesg for errors: `dmesg | grep tty`
4. Try different USB cable

### Timeout Errors

**Problem:** Operations timeout frequently.

**Solutions:**
1. Increase timeout in Lua scripts
2. Check baud rate matches device
3. Verify cable quality (especially for long cables)
4. Reduce data rate

### Encoding Issues

**Problem:** Received data looks garbled.

**Solutions:**
1. Verify baud rate matches device
2. Check data bits, stop bits, parity
3. Ensure correct protocol is selected
4. Use hex display: `log_debug("Hex: " .. string_to_hex(data))`

### Lua Script Errors

**Problem:** Script fails with error.

**Debugging:**
```lua
-- Add extensive logging
log_info("Starting step 1")
-- ... code ...
log_info("Step 1 complete")

-- Use pcall for error handling
local success, err = pcall(function()
    -- risky code
    port:write(data)
end)

if not success then
    log_error("Error: " .. tostring(err))
end
```

### Performance Issues

**Problem:** Slow data transfer.

**Solutions:**
1. Use LuaJIT (default)
2. Minimize logging in tight loops
3. Use batch operations
4. Increase baud rate if hardware supports it

## Examples

### Example 1: AT Command Device

```lua
-- examples/at_commands.lua

log_info("AT Command Test")

local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200
})

-- Test basic AT
port:write("AT\r\n")
sleep_ms(100)
local resp = port:read_until(string.byte("\n"))
log_info("Response: " .. resp)

-- Get device info
port:write("AT+GMI\r\n")  -- Manufacturer ID
sleep_ms(100)
resp = port:read_until(string.byte("\n"))
log_info("Manufacturer: " .. resp)

port:close()
```

### Example 2: Modbus RTU Communication

```lua
-- examples/modbus_test.lua

log_info("Modbus RTU Test")

local port = serial.open("/dev/ttyUSB0", {
    baudrate = 9600
})

-- Read holding registers (function code 0x03)
local request = string.char(
    0x01,  -- Slave ID
    0x03,  -- Function code: Read Holding Registers
    0x00,  -- Address high
    0x00,  -- Address low
    0x00,  -- Quantity high
    0x0A   -- Quantity low (10 registers)
)

-- Protocol will add CRC16
port:write(request)

-- Read response
sleep_ms(100)
local response = port:read(100)

log_info("Response (hex): " .. string_to_hex(response))

port:close()
```

### Example 3: Binary Data Transfer

```lua
-- Binary data example

log_info("Binary Transfer Test")

local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200
})

-- Send binary packet
local packet = string.char(
    0xAA, 0x55,        -- Header
    0x03,              -- Length
    0x01, 0x02, 0x03,  -- Data
    0x00               -- Checksum (simplified)
)

port:write(packet)

-- Read binary response
local raw = port:read_exact(10)

-- Process as hex
log_info("Received: " .. string_to_hex(raw))

port:close()
```

### Example 4: Continuous Monitoring

```lua
-- Monitoring example

log_info("Starting monitor")

local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200
})

-- Monitor for 60 seconds
local end_time = timestamp_ms() + 60000

while timestamp_ms() < end_time do
    -- Check for data
    local data = port:read(100)  -- 100ms timeout

    if data and #data > 0 then
        local hex = string_to_hex(data)
        log_info("Received: " .. hex)

        -- Check for specific pattern
        if string.find(hex, " AABBCCDD ") then
            log_info("Found pattern!")
            break
        end
    end
end

port:close()
log_info("Monitor complete")
```

### Example 5: Multi-Port Communication

```lua
-- Multi-port example

log_info("Multi-port Test")

-- Open two ports
local port1 = serial.open("/dev/ttyUSB0", {baudrate = 115200})
local port2 = serial.open("/dev/ttyUSB1", {baudrate = 9600})

-- Send to port 1
port1:write("AT+CMD1\r\n")

-- Send to port 2
port2:write("AT+CMD2\r\n")

sleep_ms(200)

-- Read from both
local resp1 = port1:read(100)
local resp2 = port2:read(100)

log_info("Port 1: " .. string_to_hex(resp1))
log_info("Port 2: " .. string_to_hex(resp2))

-- Close both
port1:close()
port2:close()
```

## Additional Resources

- [README.md](README.md) - Project overview
- [DEVELOPMENT.md](DEVELOPMENT.md) - Development guide
- [CROSS_COMPILE.md](CROSS_COMPILE.md) - Cross-compilation guide
- [examples/](examples/) - More Lua script examples
- [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) - Detailed troubleshooting

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/yourusername/serial-cli/issues
- Documentation: https://github.com/yourusername/serial-cli/wiki

---

**Serial CLI v0.1.0** - Universal Serial Port Tool Optimized for AI Interaction

### Protocol Management

#### `protocol load <path>`

Load a custom protocol from a Lua script file.

```bash
serial-cli protocol load /path/to/custom.lua
serial-cli protocol load --name my_proto /path/to/custom.lua
```

#### `protocol unload <name>`

Unload a custom protocol.

```bash
serial-cli protocol unload my_proto
```

#### `protocol reload <name>`

Reload a protocol from its file.

```bash
serial-cli protocol reload my_proto
```

#### `protocol list`

List all available protocols.

```bash
serial-cli protocol list
serial-cli protocol list --verbose
```

#### `protocol info <name>`

Show detailed information about a protocol.

```bash
serial-cli protocol info my_proto
```

#### `protocol validate <path>`

Validate a protocol script without loading it.

```bash
serial-cli protocol validate /path/to/custom.lua
```

### Lua Protocol API

#### `protocol_load(path)`

Load a protocol from a Lua script file.

```lua
local ok, err = protocol_load("/path/to/custom.lua")
if not ok then
    log_error("Failed to load protocol: " .. err)
end
```

#### `protocol_unload(name)`

Unload a protocol.

```lua
local ok, err = protocol_unload("my_proto")
```

#### `protocol_reload(name)`

Reload a protocol.

```lua
local ok, err = protocol_reload("my_proto")
```
