#!/usr/bin/env serial-cli run
-- modbus_with_tools.lua: Modbus RTU example using protocol tools
--
-- This script demonstrates how to use the built-in Modbus protocol tools
-- for communicating with Modbus RTU devices. The protocol tools handle
-- frame formatting, CRC calculation, and response parsing automatically.
--
-- Usage:
--   serial-cli run modbus_with_tools.lua --port=/dev/ttyUSB0 --baudrate=9600

-- Parse command line arguments
local args = {...}
local port_name = args[1] or "/dev/ttyUSB0"
local baudrate = tonumber(args[2]) or 9600
local device_id = tonumber(args[3]) or 1

-- Open serial port with Modbus-appropriate settings
local port = serial.open(port_name, {
    baudrate = baudrate,
    timeout = 1000,
    -- Modbus RTU typically uses 8E1 (8 data bits, even parity, 1 stop bit)
    data_bits = 8,
    parity = "even",
    stop_bits = 1
})

log_info("Modbus RTU Tool")
log_info("Port: " .. port_name .. " @ " .. baudrate)
log_info("Device ID: " .. device_id)

-- Create Modbus protocol handler
local modbus = serial.protocols.modbus.new(port, {
    device_id = device_id,
    timeout = 1000,
    retries = 3
})

-- Helper function to display register values
local function display_registers(start_addr, values)
    log_info("Registers [" .. start_addr .. "-" .. (start_addr + #values - 1) .. "]:")
    for i, value in ipairs(values) do
        local addr = start_addr + i - 1
        log_info(string.format("  [%04d]: 0x%04X (%d)", addr, value, value))
    end
end

-- Example 1: Read holding registers
log_info("\n--- Example 1: Read Holding Registers ---")
local success, registers = pcall(function()
    return modbus:read_holding_registers(0x0000, 10)  -- Read 10 registers starting at 0x0000
end)

if success then
    display_registers(0x0000, registers)
else
    log_error("Failed to read holding registers: " .. tostring(registers))
end

-- Example 2: Read input registers
log_info("\n--- Example 2: Read Input Registers ---")
local success, input_regs = pcall(function()
    return modbus:read_input_registers(0x0000, 5)  -- Read 5 input registers
end)

if success then
    display_registers(0x0000, input_regs)
else
    log_error("Failed to read input registers: " .. tostring(input_regs))
end

-- Example 3: Write single register
log_info("\n--- Example 3: Write Single Register ---")
local write_addr = 0x0001
local write_value = 0x0042
local success, result = pcall(function()
    return modbus:write_single_register(write_addr, write_value)
end)

if success then
    log_info(string.format("Wrote register [%04d] = 0x%04X (%d)", write_addr, write_value, write_value))
else
    log_error("Failed to write register: " .. tostring(result))
end

-- Example 4: Write multiple registers
log_info("\n--- Example 4: Write Multiple Registers ---")
local multi_addr = 0x0010
local multi_values = {0x0001, 0x0002, 0x0003, 0x0004, 0x0005}
local success, result = pcall(function()
    return modbus:write_multiple_registers(multi_addr, multi_values)
end)

if success then
    log_info("Wrote " .. #multi_values .. " registers starting at [" .. multi_addr .. "]")
    display_registers(multi_addr, multi_values)
else
    log_error("Failed to write multiple registers: " .. tostring(result))
end

-- Example 5: Read device info (if supported)
log_info("\n--- Example 5: Report Slave ID ---")
local success, slave_id = pcall(function()
    return modbus:report_slave_id()
end)

if success then
    log_info("Slave ID: " .. slave_id)
else
    log_info("Device doesn't support Report Slave ID (optional feature)")
end

-- Clean up
modbus:close()
port:close()
log_info("\nModbus communication complete. Port closed.")
