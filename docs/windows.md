# Windows Platform Guide

Complete guide for using serial-cli on Windows platforms.

## Quick Start

### Installation

```bash
# From source
cargo install --path .

# Or download pre-built binaries from releases
```

### Basic Usage

```bash
# List available ports
serial-cli list-ports

# Interactive mode
serial-cli interactive

# Run Lua script
serial-cli run script.lua
```

## Windows-Specific Features

### 1. Flow Control

Support for hardware (RTS/CTS) and software (XON/XOFF) flow control:

```lua
local port = serial_open("COM3", {
    baudrate = 115200,
    flow_control = "hardware"  -- RTS/CTS
})

-- Software flow control
local port = serial_open("COM1", {
    flow_control = "software"  -- XON/XOFF
})
```

### 2. Signal Control

Configure DTR/RTS signals at port open:

```lua
local port = serial_open("COM3", {
    dtr_enable = true,  -- Enable DTR
    rts_enable = true   -- Enable RTS
})
```

### 3. Enhanced Error Messages

Get helpful suggestions for common Windows issues:

```bash
# Permission error
Permission denied for port 'COM3': Try running as Administrator

# Port busy error
Port 'COM1' is already in use: Close other applications using this port
```

### 4. Port Information

Detailed port enumeration with COM numbers:

```lua
local ports = serial_list()
for _, port in ipairs(ports) do
    print(port.port_name)      -- "COM3"
    print(port.com_number)     -- 3
    print(port.friendly_name)  -- "USB Serial Port"
end
```

## Configuration Reference

### Serial Port Settings

```lua
local port = serial_open("COM3", {
    baudrate = 115200,       -- Communication speed
    data_bits = 8,           -- 5, 6, 7, or 8
    stop_bits = 1,           -- 1 or 2
    parity = "none",         -- "none", "odd", "even"
    flow_control = "none",   -- "none", "software", "hardware"
    timeout = 1000,          -- Timeout in milliseconds
    dtr_enable = true,       -- DTR signal
    rts_enable = true        -- RTS signal
})
```

### Common Baud Rates

- **Low Speed**: 300, 1200, 2400, 4800, 9600
- **Medium Speed**: 19200, 38400, 57600
- **High Speed**: 115200, 230400, 460800, 921600

### Parity Settings

- `"none"` - No parity (most common)
- `"odd"` - Odd parity
- `"even"` - Even parity (Modbus RTU)

## Usage Examples

### Basic Serial Communication

```lua
local port = serial_open("COM3", {
    baudrate = 115200
})

port:write("AT\r\n")
local response = port:read(256)
log_info("Received: " .. response)

serial_close(port)
```

### Modbus RTU with Even Parity

```lua
local port = serial_open("COM1", {
    baudrate = 9600,
    parity = "even",
    flow_control = "none"
})

local modbus = serial_protocols_modbus_new(port, {
    device_id = 1
})

local registers = modbus:read_holding_registers(0x0000, 10)
for i, value in ipairs(registers) do
    log_info("Register " .. i .. ": " .. value)
end
```

### Industrial PLC Communication

```lua
local port = serial_open("COM4", {
    baudrate = 19200,
    data_bits = 7,
    stop_bits = 2,
    parity = "even",
    flow_control = "hardware",
    timeout = 2000
})

port:write("\x02PING\x03\x45")
local response = port:read(256)
```

## Troubleshooting

### Permission Denied

**Error**: `Permission denied for port 'COM3'`

**Solutions**:
1. Run Command Prompt/PowerShell as Administrator
2. Check if another application is using the port
3. Verify Windows user permissions for COM ports

### Port Not Found

**Error**: `Port 'COM999' not found`

**Solutions**:
1. List available ports: `serial-cli list-ports`
2. Check Device Manager → Ports (COM & LPT)
3. Install USB-to-Serial drivers if needed

### Port Busy

**Error**: `Port 'COM1' is already in use`

**Solutions**:
1. Close PuTTY, Tera Term, Arduino IDE, etc.
2. Use Device Manager to disable and re-enable the port
3. Restart computer if port remains busy

### No Response

**Issue**: Port opens but no data received

**Solutions**:
1. Verify baud rate and configuration match device
2. Check cable connections
3. Try longer timeout: `timeout = 5000`
4. Enable verbose logging: `serial-cli --verbose run script.lua`

## Windows Device Manager

### Check Port Status

1. Press `Win+X`, select "Device Manager"
2. Expand "Ports (COM & LPT)"
3. Right-click port → Properties for details

### Install Drivers

- **USB-to-Serial**: FTDI, CP210x, CH340 drivers
- **Arduino**: Install Arduino IDE (includes drivers)
- **Other**: Check manufacturer website

## Tips and Best Practices

### High-Speed Communication

```lua
local port = serial_open("COM1", {
    baudrate = 921600,      -- Maximum speed
    timeout = 50,           -- Short timeout
    flow_control = "hardware"
})
```

### Reliable Connections

```lua
local port = serial_open("COM1", {
    baudrate = 9600,
    timeout = 5000,         -- Long timeout
    flow_control = "software"
})
```

### Error Handling

```lua
local ok, result = pcall(serial_open, "COM3", {baudrate = 115200})
if not ok then
    log_error("Failed to open port: " .. tostring(result))
    return
end

local port = result
-- Use port...
serial_close(port)
```

## PowerShell Integration

```powershell
# List ports as JSON
$ports = serial-cli list-ports | ConvertFrom-Json
$ports | Format-Table port_name, com_number, friendly_name

# Run script with error handling
serial-cli run script.lua
if ($LASTEXITCODE -ne 0) {
    Write-Error "Script failed"
}
```

## Additional Resources

- **[USAGE.md](../USAGE.md)** - Complete usage guide
- **[TODO_WINDOWS.md](../TODO_WINDOWS.md)** - Windows improvements roadmap
- **[examples/windows_serial_example.lua](../examples/windows_serial_example.lua)** - Example script

## Common Windows Devices

| Device Type | Typical Settings | Flow Control |
|-------------|------------------|--------------|
| Arduino | 115200 8N1 | None |
| Modbus RTU | 9600 8E1 | None |
| Industrial PLC | 9600-19200 7E2 | Hardware |
| GPS Receiver | 4800-9600 8N1 | None |
| Bluetooth SPP | 9600 8N1 | Hardware |

**Legend**: `8N1` = 8 data bits, No parity, 1 stop bit

---

**Last Updated**: 2026-04-02
**Windows Support**: ✅ Windows 10/11
**Status**: Production Ready
