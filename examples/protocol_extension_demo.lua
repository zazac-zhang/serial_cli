#!/usr/bin/env serial-cli run
-- Lua Protocol Extension Demo
--
-- This script demonstrates the complete protocol extension API

-- 1. Load a custom protocol
log_info("=== Protocol Extension Demo ===")

local ok, err = protocol_load("examples/custom_protocol.lua")
if ok then
    log_info("✓ Custom protocol loaded successfully")
else
    log_error("✗ Failed to load custom protocol: " .. tostring(err))
    return
end

-- 2. List all available protocols
local protocols = protocol_list()

log_info("Available protocols:")
for i, proto in ipairs(protocols) do
    local info = protocol_info(proto.name)
    local type_mark = info.type == "custom" and "(custom)" or "(built-in)"
    log_info(string.format("  %d. %s %s", i, proto.name, type_mark))
end

-- 3. Use the protocol
local data = hex_to_bytes("01 02 03 04 05")
local encoded = protocol_encode("custom_protocol", bytes_to_string(data))

log_info("Original data: " .. bytes_to_hex(data))
log_info("Encoded data: " .. encoded)

local decoded = protocol_decode("custom_protocol", encoded)
log_info("Decoded data: " .. bytes_to_hex(hex_to_bytes(decoded)))

-- 4. Demonstrate data conversion utilities
log_info("\n=== Data Conversion Demo ===")

local hex_str = "48 65 6C 6C 6F"
local bytes = hex_to_bytes(hex_str)
local str = bytes_to_string(bytes)

log_info("Hex to string: " .. hex_str .. " → " .. str)

local original = "Test Data"
local back_to_hex = bytes_to_hex(string_to_bytes(original))
log_info("String to hex: " .. original .. " → " .. back_to_hex)

-- 5. Test protocol validation
log_info("\n=== Protocol Validation Demo ===")

ok, err = protocol_validate("examples/custom_protocol.lua")
if ok then
    log_info("✓ Protocol validation passed")
else
    log_error("✗ Protocol validation failed: " .. tostring(err))
end

-- Test with invalid protocol
ok, err = protocol_validate("tests/fixtures/protocols/test_syntax_error.lua")
if not ok then
    log_info("✓ Correctly rejected invalid protocol: " .. tostring(err))
end

log_info("\n=== Demo Complete ===")
