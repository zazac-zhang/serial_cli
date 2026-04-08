-- Windows Serial Communication Example
-- Demonstrates the use of new serial port features on Windows

-- Open COM3 port with enhanced configuration
local port = serial_open("COM3", {
    baudrate = 115200,           -- Communication speed
    data_bits = 8,               -- 8 data bits
    stop_bits = 1,               -- 1 stop bit
    parity = "none",             -- No parity
    flow_control = "hardware",   -- RTS/CTS hardware flow control
    timeout = 1000,              -- 1 second timeout
    dtr_enable = true,           -- Enable DTR signal
    rts_enable = true            -- Enable RTS signal
})

if not port then
    log_error("Failed to open port")
    return
end

log_info("Port opened successfully")

-- Send AT command (common for modems)
log_info("Sending AT command...")
port:write("AT\r\n")

-- Wait a bit for response
sleep_ms(500)

-- Read response
local response = port:read(256)
if response then
    log_info("Received: " .. response)
else
    log_warn("No response received")
end

-- Close the port
serial_close(port)
log_info("Port closed")
