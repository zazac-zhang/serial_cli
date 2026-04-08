#!/usr/bin/env serial-cli run
-- custom_protocol.lua: Custom protocol implementation example
--
-- This script demonstrates how to implement a custom protocol handler
-- using the Lua serial API. It shows proper frame parsing, validation,
-- and error handling for a simple binary protocol.
--
-- Protocol specification:
--   Frame format: [START][CMD][LEN][DATA...][CRC][END]
--   START: 0xAA (1 byte)
--   CMD:   Command ID (1 byte)
--   LEN:   Data length (1 byte, 0-255)
--   DATA:  Payload (LEN bytes)
--   CRC:   CRC-8 checksum (1 byte)
--   END:   0x55 (1 byte)
--
-- Usage:
--   serial-cli run custom_protocol.lua --port=/dev/ttyUSB0 --baudrate=115200

-- Parse command line arguments
local args = {...}
local port_name = args[1] or "/dev/ttyUSB0"
local baudrate = tonumber(args[2]) or 115200

-- Open serial port
local port = serial.open(port_name, {
    baudrate = baudrate,
    timeout = 1000
})

log_info("Custom Protocol Example")
log_info("Port: " .. port_name .. " @ " .. baudrate)

-- Protocol constants
local START_BYTE = 0xAA
local END_BYTE = 0x55

-- CRC-8 calculation (polynomial 0x07, initial value 0x00)
local function calculate_crc8(data)
    local crc = 0x00
    for i = 1, #data do
        crc = bit.bxor(crc, data:byte(i))
        for j = 1, 8 do
            if bit.band(crc, 0x80) ~= 0 then
                crc = bit.bxor(bit.lshift(crc, 1), 0x07)
            else
                crc = bit.lshift(crc, 1)
            end
        end
    end
    return bit.band(crc, 0xFF)
end

-- Build a protocol frame
local function build_frame(cmd, data)
    local frame = string.char(START_BYTE, cmd, #data)
    if #data > 0 then
        frame = frame .. data
    end
    local crc = calculate_crc8(frame:sub(2))  -- CRC from CMD to end of data
    frame = frame .. string.char(crc, END_BYTE)
    return frame
end

-- Parse a protocol frame
local function parse_frame(frame)
    -- Validate minimum frame length
    if #frame < 5 then
        return nil, "Frame too short"
    end

    -- Check start byte
    if frame:byte(1) ~= START_BYTE then
        return nil, "Invalid start byte"
    end

    -- Extract header
    local cmd = frame:byte(2)
    local len = frame:byte(3)

    -- Validate length
    if #frame < (5 + len) then
        return nil, "Invalid length field"
    end

    -- Extract data
    local data = frame:sub(4, 4 + len - 1)

    -- Extract and verify CRC
    local crc_received = frame:byte(4 + len)
    local crc_calculated = calculate_crc8(frame:sub(2, 4 + len - 1))

    if crc_received ~= crc_calculated then
        return nil, string.format("CRC mismatch: got 0x%02X, expected 0x%02X",
                                 crc_received, crc_calculated)
    end

    -- Check end byte
    if frame:byte(5 + len) ~= END_BYTE then
        return nil, "Invalid end byte"
    end

    -- Return parsed frame
    return {
        cmd = cmd,
        data = data,
        length = len
    }
end

-- Send a command and wait for response
local function send_command(cmd, data, timeout_ms)
    timeout_ms = timeout_ms or 1000

    -- Build and send frame
    local frame = build_frame(cmd, data)
    log_info(string.format("Sending: CMD=0x%02X, LEN=%d", cmd, #data))
    port:write(frame)

    -- Wait for response
    local start_time = os.time()
    local response_buffer = ""

    while (os.time() - start_time) * 1000 < timeout_ms do
        local chunk = port:read(256)
        if chunk and #chunk > 0 then
            response_buffer = response_buffer .. chunk

            -- Try to parse frame
            local frame, err = parse_frame(response_buffer)
            if frame then
                return frame
            elseif err then
                -- Clear buffer on error and wait for more data
                if #response_buffer > 256 then  -- Prevent buffer overflow
                    log_warn("Response buffer overflow, clearing")
                    response_buffer = ""
                end
            end
        end
        sleep_ms(10)
    end

    return nil, "Timeout waiting for response"
end

-- Example commands
local CMD_PING = 0x01
local CMD_GET_STATUS = 0x02
local CMD_SET_CONFIG = 0x03
local CMD_READ_DATA = 0x04

-- Example 1: Ping device
log_info("\n--- Example 1: Ping Device ---")
local success, response = pcall(function()
    return send_command(CMD_PING, "")
end)

if success and response then
    log_info(string.format("Pong: CMD=0x%02X, DATA_LEN=%d", response.cmd, response.length))
else
    log_error("Ping failed: " .. tostring(response))
end

-- Example 2: Get device status
log_info("\n--- Example 2: Get Device Status ---")
local success, response = pcall(function()
    return send_command(CMD_GET_STATUS, "")
end)

if success and response then
    log_info(string.format("Status: CMD=0x%02X", response.cmd))
    if response.length >= 4 then
        local status = string.byte(response.data, 1)
        local temp = string.byte(response.data, 2)
        local voltage = string.byte(response.data, 3)
        local flags = string.byte(response.data, 4)
        log_info(string.format("  Status: 0x%02X, Temp: %d°C, Voltage: %dV, Flags: 0x%02X",
                             status, temp, voltage, flags))
    end
else
    log_error("Get status failed: " .. tostring(response))
end

-- Example 3: Set configuration
log_info("\n--- Example 3: Set Configuration ---")
local config_data = string.char(0x01, 0x64, 0x00, 0x0A)  -- Example config
local success, response = pcall(function()
    return send_command(CMD_SET_CONFIG, config_data)
end)

if success and response then
    log_info(string.format("Config set: CMD=0x%02X, ACK=%s",
                         response.cmd, response.data:byte(1) == 0x00))
else
    log_error("Set config failed: " .. tostring(response))
end

-- Example 4: Read data block
log_info("\n--- Example 4: Read Data Block ---")
local read_params = string.char(0x00, 0x10)  -- Read 16 bytes from address 0x00
local success, response = pcall(function()
    return send_command(CMD_READ_DATA, read_params)
end)

if success and response then
    log_info(string.format("Data read: CMD=0x%02X, LEN=%d", response.cmd, response.length))
    if response.length > 0 then
        log_info("Data (hex): " .. response.data:gsub(".", function(c)
            return string.format("%02X ", string.byte(c))
        end))
    end
else
    log_error("Read data failed: " .. tostring(response))
end

-- Clean up
port:close()
log_info("\nCustom protocol examples complete. Port closed.")
