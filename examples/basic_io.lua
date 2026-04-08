-- Basic I/O example
-- This script demonstrates basic serial port I/O

-- Open a serial port
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200,
    databits = 8,
    stopbits = 1,
    parity = "none"
})

-- Send data
port:write("AT\r\n")

-- Read response
local response = port:read_until("OK")
print("Response: " .. response)

-- Close the port
port:close()
