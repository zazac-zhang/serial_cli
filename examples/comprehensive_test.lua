-- Comprehensive test example
-- This script demonstrates various features of serial-cli

-- Test 1: Basic logging
log.info("Starting comprehensive test")

-- Test 2: Hex encoding/decoding
local original = {0xAA, 0xBB, 0xCC}
local encoded = hex.encode(original)
log.info("Encoded: " .. encoded)

local decoded = hex.decode(encoded)
log.info("Decoded length: " .. #decoded)

-- Test 3: Simple math operations
local result = 2 + 2
assert(result == 4, "Math failed")

log.info("Test completed successfully!")
