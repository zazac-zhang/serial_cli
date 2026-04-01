-- AT Commands example
-- This script demonstrates AT command usage

-- Open port
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 115200
})

-- Set protocol to AT Command
port:set_protocol("at_command")

-- Send AT command
port:write("AT")
local response = port:read_until("OK")
print("AT Response: " .. response)

-- Get module info
port:write("ATI")
response = port:read_until("OK")
print("Module Info: " .. response)

-- Close the port
port:close()
