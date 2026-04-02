#!/usr/bin/env serial-cli run
-- raw_echo.lua: Simple echo example without protocol handling
--
-- This script demonstrates basic serial I/O without using any protocol
-- tools. It's useful for simple devices that don't require structured
-- communication protocols.
--
-- Usage:
--   serial-cli run raw_echo.lua --port=/dev/ttyUSB0 --baudrate=115200

-- Parse command line arguments
local args = {...}
local port_name = args[1] or "/dev/ttyUSB0"
local baudrate = tonumber(args[2]) or 115200

-- Open serial port with configuration
local port = serial.open(port_name, {
    baudrate = baudrate,
    timeout = 1000  -- 1 second timeout for reads
})

log_info("Opened port: " .. port_name)
log_info("Baudrate: " .. baudrate)

-- Send a test message
local test_msg = "Hello, Serial!\r\n"
log_info("Sending: " .. test_msg:gsub("\r", "\\r"):gsub("\n", "\\n"))
port:write(test_msg)

-- Wait for response
sleep_ms(100)

-- Read response (up to 256 bytes or until timeout)
local response = port:read(256)
if response and #response > 0 then
    log_info("Received (" .. #response .. " bytes): " .. response)
else
    log_warn("No response received")
end

-- Echo loop: read and echo back data for 5 iterations
for i = 1, 5 do
    log_info("Iteration " .. i .. " - waiting for data...")

    -- Wait for incoming data
    sleep_ms(500)

    -- Read available data
    local data = port:read(128)
    if data and #data > 0 then
        log_info("Read (" .. #data .. " bytes): " .. data)

        -- Echo the data back
        port:write(data)
        log_info("Echoed back")
    else
        log_info("No data available")
    end
end

-- Clean up
port:close()
log_info("Port closed. Goodbye!")
