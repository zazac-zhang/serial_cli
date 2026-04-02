# Windows Serial Port Examples

This file demonstrates how to use serial-cli on Windows with the new enhanced features.

## Basic Usage

### List Available Ports

```bash
serial-cli list-ports
```

Example output:
```json
[
  {
    "port_name": "COM1",
    "port_type": "UsbPort",
    "friendly_name": "USB Serial Port",
    "com_number": 1
  },
  {
    "port_name": "COM3",
    "port_type": "PciPort",
    "com_number": 3
  }
]
```

### Open Port with Basic Configuration

```bash
serial-cli interactive
serial> open COM3
Opened port: uuid-12345678
```

## Advanced Configuration

### Lua Script with Flow Control

Create a file `windows_flow_control.lua`:

```lua
local port = serial_open("COM3", {
    baudrate = 115200,
    data_bits = 8,
    stop_bits = 1,
    parity = "none",
    flow_control = "hardware",  -- RTS/CTS flow control
    timeout = 1000,
    dtr_enable = true,
    rts_enable = true
})

-- Send data
port:write("AT\r\n")

-- Read response
local response = port:read(256)
log_info("Received: " .. response)

serial_close(port)
```

Run the script:
```bash
serial-cli run windows_flow_control.lua
```

### Software Flow Control (XON/XOFF)

```lua
local port = serial_open("COM1", {
    baudrate = 9600,
    flow_control = "software",  -- XON/XOFF flow control
    timeout = 5000
})
```

### No Flow Control

```lua
local port = serial_open("COM1", {
    baudrate = 115200,
    flow_control = "none"
})
```

## Modbus RTU on Windows

### Modbus with Even Parity

```lua
local port = serial_open("COM3", {
    baudrate = 9600,
    data_bits = 8,
    stop_bits = 1,
    parity = "even",        -- Modbus RTU typically uses even parity
    flow_control = "none",  -- Modbus doesn't use flow control
    timeout = 1000
})

-- Use Modbus protocol functions
local modbus = serial_protocols_modbus_new(port, {
    device_id = 1,
    timeout = 1000
})

-- Read holding registers
local registers = modbus:read_holding_registers(0x0000, 10)
for i, value in ipairs(registers) do
    log_info("Register " .. i .. ": " .. value)
end

modbus:close()
serial_close(port)
```

## Error Handling

### Permission Errors

If you see this error:
```
Permission denied for port 'COM3': Try running as Administrator or check port permissions
```

**Solutions**:
1. Run Command Prompt or PowerShell as Administrator
2. Check if another application is using the port
3. Verify Windows user permissions for COM ports

### Port Busy Errors

If you see this error:
```
Port 'COM3' is already in use: Close other applications using this port or try a different port
```

**Solutions**:
1. Close applications like PuTTY, Tera Term, or Arduino IDE
2. Use Device Manager to disable and re-enable the port
3. Restart the computer if the port remains busy

### Port Not Found Errors

If you see this error:
```
Port 'COM999' not found
```

**Solutions**:
1. List available ports with `serial-cli list-ports`
2. Check Device Manager for available COM ports
3. Install USB-to-Serial drivers if needed

## Windows-Specific Considerations

### COM Port Numbers

Windows uses COM port names (COM1, COM2, etc.):

```lua
-- Correct
local port = serial_open("COM3", {baudrate = 115200})

-- Incorrect - Unix style
local port = serial_open("/dev/ttyUSB0", {baudrate = 115200})
```

### Baud Rates

Common Windows baud rates:
- 110, 300, 600, 1200, 2400, 4800, 9600
- 14400, 19200, 38400, 57600, 115200
- 128000, 256000 (some USB serial adapters)

```lua
-- Standard baud rate
local port = serial_open("COM1", {baudrate = 115200})

-- High-speed USB serial adapter
local port = serial_open("COM3", {baudrate = 921600})
```

### Timeout Configuration

Windows serial ports may require longer timeouts:

```lua
-- Short timeout (fast responses)
local port = serial_open("COM1", {timeout = 100})

-- Long timeout (slow devices)
local port = serial_open("COM1", {timeout = 5000})

-- No timeout (blocking read)
-- Note: Not recommended for interactive scripts
local port = serial_open("COM1", {timeout = 0})
```

## Troubleshooting

### Check Port Status

Use Windows Device Manager:
1. Press Win+X, select "Device Manager"
2. Expand "Ports (COM & LPT)"
3. Check port status and properties

### Enable Verbose Logging

```bash
serial-cli --verbose run script.lua
```

### Test with Windows Tools

Before using serial-cli, test with Windows tools:
- **PuTTY**: Serial terminal emulation
- **Tera Term**: Alternative terminal
- **Arduino IDE**: Serial monitor for Arduino devices

## Advanced Examples

### Industrial PLC Communication

```lua
-- Connect to industrial PLC with specific settings
local port = serial_open("COM4", {
    baudrate = 19200,
    data_bits = 7,
    stop_bits = 2,
    parity = "even",
    flow_control = "hardware",
    timeout = 2000,
    dtr_enable = true,      -- Some PLCs require DTR
    rts_enable = true       -- Some PLCs require RTS
})

-- Send command
port:write("\x02PING\x03\x45")  -- STX + "PING" + ETX + checksum

-- Wait for response
local response = port:read(256)
log_info("PLC Response: " .. response)

serial_close(port)
```

### USB Serial Device with Custom Settings

```lua
-- USB-to-Serial adapter with non-standard settings
local port = serial_open("COM10", {
    baudrate = 921600,      -- High-speed USB
    data_bits = 8,
    stop_bits = 1,
    parity = "none",
    flow_control = "hardware",
    timeout = 100,
    dtr_enable = false,     -- Disable for this device
    rts_enable = true
})

-- Binary data transmission
local data = string.char(0xAA, 0x55, 0x01, 0x02, 0x03)
port:write(data)

serial_close(port)
```

## Performance Tips

### Reduce Latency

```lua
-- For high-speed communication, use shorter timeouts
local port = serial_open("COM1", {
    baudrate = 115200,
    timeout = 50,           -- Short timeout
    flow_control = "hardware"
})
```

### Improve Reliability

```lua
-- For unreliable connections, use longer timeouts
local port = serial_open("COM1", {
    baudrate = 9600,
    timeout = 5000,         -- Long timeout
    flow_control = "software"  -- XON/XOFF
})
```

## Integration with Windows Tools

### PowerShell Automation

```powershell
# Run serial script from PowerShell
serial-cli run script.lua

# Check available ports
$ports = serial-cli list-ports | ConvertFrom-Json
$ports | ForEach-Object { Write-Host "Port: $($_.port_name)" }
```

### Batch Files

```batch
@echo off
REM Automate serial communication
serial-cli run automation.lua
if errorlevel 1 (
    echo Error occurred
    exit /b 1
)
echo Success
exit /b 0
```

## Windows Event Logging

When errors occur, check Windows Event Viewer:
1. Press Win+R, type `eventvwr`
2. Look for application errors related to serial ports
3. Check System logs for driver issues

## Additional Resources

- [USAGE.md](USAGE.md) - General usage guide
- [TODO_WINDOWS.md](TODO_WINDOWS.md) - Windows improvements roadmap
- [CROSS_COMPILE.md](CROSS_COMPILE.md) - Cross-compilation guide

---

**Last Updated**: 2026-04-02
**Windows Version**: 10/11
