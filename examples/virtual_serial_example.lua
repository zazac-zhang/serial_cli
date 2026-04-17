-- Virtual Serial Port Example Script
-- This script demonstrates how to create and use virtual serial ports

-- Enable logging
log_info("Virtual Serial Port Example")
log_info("============================")

-- Create a virtual serial port pair
log_info("Creating virtual serial port pair...")

local result = virtual_create("pty", false)

if result then
    log_info("Virtual pair created successfully!")
    log_info("ID: " .. result.id)
    log_info("Port A: " .. result.port_a)
    log_info("Port B: " .. result.port_b)
    log_info("Backend: " .. result.backend)
    log_info("Running: " .. tostring(result.running))

    log_info("")
    log_info("You can now use these ports in other terminals:")
    log_info("  Terminal 1: serial-cli interactive --port " .. result.port_a)
    log_info("  Terminal 2: serial-cli interactive --port " .. result.port_b)

    -- Note: In this simple example, the virtual pair is cleaned up immediately
    -- In a real application, you'd want to keep a reference to it
    log_info("")
    log_info("Note: This example creates the pair but doesn't keep it alive.")
    log_info("For persistent virtual pairs, use the CLI command:")
    log_info("  serial-cli virtual create")
else
    log_error("Failed to create virtual pair")
end

log_info("")
log_info("Example completed")
