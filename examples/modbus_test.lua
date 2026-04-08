-- Modbus test example
-- This script demonstrates Modbus protocol usage

-- Open port with Modbus RTU protocol
local port = serial.open("/dev/ttyUSB0", {
    baudrate = 9600,
    databits = 8,
    stopbits = 1,
    parity = "none"
})

-- Set protocol to Modbus RTU
port:set_protocol("modbus_rtu")

-- Read a register
local result = port:read_register(1, 100)
print("Register value: " .. result)

-- Write a register
port:write_register(1, 100, 500)

-- Close the port
port:close()
