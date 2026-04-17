-- Virtual Serial Port Protocol Test
-- This script demonstrates protocol testing with virtual serial ports

log_info("Virtual Serial Port Protocol Test")
log_info("==================================")

-- Create virtual serial port pair with monitoring
log_info("Creating virtual serial port pair with monitoring...")

local result = virtual_create("pty", true)

if result then
    log_info("✓ Virtual pair created!")
    log_info("  Port A: " .. result.port_a)
    log_info("  Port B: " .. result.port_b)

    -- Note: In a real implementation, you would:
    -- 1. Keep the virtual pair alive
    -- 2. Open both ports using serial_open()
    -- 3. Send test data using serial_send()
    -- 4. Receive data using serial_recv()
    -- 5. Test protocol parsing

    log_info("")
    log_info("In a complete implementation, this script would:")
    log_info("  1. Open " .. result.port_a .. " for device simulation")
    log_info("  2. Open " .. result.port_b .. " for protocol testing")
    log_info("  3. Send test data and verify responses")
    log_info("  4. Validate protocol parsing")

    log_info("")
    log_info("For now, use the CLI to test protocols:")
    log_info("  1. Create: serial-cli virtual create --monitor")
    log_info("  2. Terminal 1: serial-cli interactive --port /dev/ttysXXX")
    log_info("  3. Terminal 2: serial-cli interactive --port /dev/ttysYYY")
    log_info("  4. Set protocol: protocol set modbus_rtu")
    log_info("  5. Send data and observe parsing")

else
    log_error("Failed to create virtual pair")
end

log_info("")
log_info("Protocol test example completed")
